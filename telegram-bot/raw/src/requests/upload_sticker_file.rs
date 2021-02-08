use crate::requests::*;
use crate::types::*;

#[derive(Debug, Clone)]
#[must_use = "requests do nothing unless sent"]
pub struct UploadStickerFile {
    user_id: UserId,
    png_sticker: InputFile,
}

impl ToMultipart for UploadStickerFile {
    fn to_multipart(&self) -> Result<Multipart, Error> {
        multipart_map! {
          self,
          (user_id (text));
          (png_sticker (raw));
        }
    }
}

impl Request for UploadStickerFile {
    type Type = MultipartRequestType<Self>;
    type Response = JsonIdResponse<File>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("uploadStickerFile"), self)
    }
}

impl UploadStickerFile {
    pub fn new<U: ToUserId, F: Into<InputFile>>(user_id: U, png_sticker: F) -> Self {
        Self {
            user_id: user_id.to_user_id(),
            png_sticker: png_sticker.into(),
        }
    }
}
