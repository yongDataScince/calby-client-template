pub mod enviroment;
pub mod response_process;
use std::{fs::write, io, path::Path};
pub mod documents {
    tonic::include_proto!("documents");
}

use std::{fs::File};
use std::io::Read;

use crate::documents::{
    documents_client::DocumentsClient,
    SaveFilesRequest,
    GetFileRequest
};

fn read_a_file(filepath: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(filepath)?;

    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    return Ok(data);
}

#[tokio::main]
async fn main() {
    let mut client = DocumentsClient::connect(
        "http://127.0.0.1:4424"
    ).await.expect("can't connect");
    
    let file = read_a_file("test-data/image.png").expect("cant't open file");

    let _req = SaveFilesRequest {
        user_id: "user-id-new".to_string(),
        file_name: "image.png".to_string(),
        doc_type: 6,
        data: file,
    };

    let req2 = GetFileRequest {
        user_id: "user-id-new".to_string(),
        doc_type: 6,
        file_id: "75736572-2d69-642d-6e65-773136353538".to_string(),
    };

    // let response = client.save_files(req).await.expect("error: response");
    // println!("{:?}", response.into_inner());
    let response2 = client.get_file(req2).await.expect("error: get files");
    let response2 = response2.into_inner();
    let save_data = response2.data;
    write(format!("test-data/response-data/{}", response2.file_name), save_data).expect("error: save resp data");
}
