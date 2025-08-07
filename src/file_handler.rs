//! File handler
use crate::errors::AppError;
use poll_promise::Promise;

/// File object
pub struct File {
    /// File path
    //pub path: String,
    /// File data
    pub data: Vec<u8>,
}

/// File Handler
#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct FileHandler {
    /// Dropped_files handler
    #[serde(skip)]
    pub dropped_files: Vec<egui::DroppedFile>,

    /// File upload handling
    #[serde(skip)]
    pub file_upload: Option<Promise<Result<File, AppError>>>,
}

impl FileHandler {
    /// Handle the file
    #[cfg(target_arch = "wasm32")]
    pub fn handle_file_open(&mut self) {
        self.file_upload = Some(Promise::spawn_local(async {
            let file_selected = rfd::AsyncFileDialog::new().pick_file().await;
            if let Some(curr_file) = file_selected {
                let buf = curr_file.read().await;
                return Ok(File {
                    //path: curr_file.file_name(),
                    data: buf,
                });
            }
            // no file selected
            Err(AppError::new_fake("Upload: no file Selected".to_string()))
        }));
    }

    /// Handle the file
    #[cfg(not(target_arch = "wasm32"))]
    pub fn handle_file_open(&mut self) {
        self.file_upload = Some(Promise::spawn_thread("slow", move || {
            if let Some(path_buf) = rfd::FileDialog::new().pick_file() {
                // read file as string
                if let Some(path) = path_buf.to_str() {
                    let path = path.to_string();
                    let buf = std::fs::read(path.clone());
                    let buf = match buf {
                        Ok(v) => v,
                        Err(e) => {
                            log::warn!("{e:?}");
                            return Err(AppError::new(e.to_string()));
                        }
                    };
                    return Ok(File {
                        //path,
                        data: buf,
                    });
                }
            }
            // no file selected
            Err(AppError::new_fake("Upload: no file Selected".to_string()))
        }))
    }

    /// Handle file upload
    fn handle_file_upload(&mut self) -> Result<Option<String>, AppError> {
        let res = match &self.file_upload {
            Some(result) => match result.ready() {
                Some(Ok(File { data, .. })) => {
                    match String::from_utf8(data.clone()) {
                        Ok(str) => {
                            self.file_upload = None; // reset file_upload
                            Ok(Some(str))
                        }
                        Err(err) => {
                            self.file_upload = None; // reset file_upload
                            Err(err.into())
                        }
                    }
                }
                Some(Err(e)) => {
                    let err = e.clone();
                    self.file_upload = None; // reset file_upload
                    Err(err)
                }
                None => Ok(None),
            },
            None => Ok(None),
        };
        res
    }

    /// Handle file dropped
    fn handle_file_dropped(&mut self) -> Result<Option<String>, AppError> {
        if self.dropped_files.is_empty() {
            return Ok(None);
        }
        let file = self.dropped_files.remove(0);
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(path) = file.path.as_deref() {
                use std::fs::read_to_string;

                let svg = read_to_string(path)?;
                return Ok(Some(svg));
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(bytes) = file.bytes.as_deref() {
                let svg_str = String::from_utf8(bytes.to_vec())?;
                return Ok(Some(svg_str));
            }
        }
        Ok(None)
    }

    /// Handle the files
    pub fn handle_files(&mut self, ctx: &egui::Context) -> Result<Option<String>, AppError> {
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                // read the first file
                self.dropped_files.clone_from(&i.raw.dropped_files);
            }
        });
        if let Some(img) = self.handle_file_upload()? {
            return Ok(Some(img));
        }
        if let Some(img) = self.handle_file_dropped()? {
            return Ok(Some(img));
        }
        Ok(None)
    }
}
