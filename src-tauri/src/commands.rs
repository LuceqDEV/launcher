use crate::{unwrap_or_err, ClientUrls};

#[tauri::command]
pub fn clienturls() -> &'static Option<ClientUrls> {
    crate::get_clienturls()
}

#[tauri::command]
pub fn check_update() -> Result<ClientUrls, &'static str> {
    let clienturls = unsafe {
        crate::helper::ref_to_mut(crate::get_clienturls())
    };
    
    let client = reqwest::blocking::Client::new();
    let res = unwrap_or_err!(
        client.get(crate::CLIENURLS_URL).header(reqwest::header::CONTENT_TYPE, "application/json").header(reqwest::header::USER_AGENT, "Mozilla/1.22 (compatible; MSIE 5.01; PalmOS 3.0) EudoraWeb 2").send(),
        "request failed"
    );
    
    let content = unwrap_or_err!(res.json::<ClientUrls>(), "content missing");
    *clienturls = Some(content.clone());
    Ok(content)
}

#[tauri::command]
pub fn install_update() -> Result<usize, &'static str> {
    let clienturls = crate::get_clienturls().as_ref().unwrap();
    
    let res = unwrap_or_err!(reqwest::blocking::get(&clienturls.shockwave_windows), "request failed");
    let content = unwrap_or_err!(res.bytes(), "missing content");
    let mut out = unwrap_or_err!(std::fs::File::create("update.zip"), "file could not be created");
    unwrap_or_err!(std::io::copy(&mut &*content, &mut out), "content could not be copied");

    let output_path = format!("versions/{}", clienturls.shockwave_windows_version);
    if std::fs::create_dir(&output_path).is_err() {
        return Err("");
    }

    let zipfile = unwrap_or_err!(std::fs::File::open("update.zip"), "file could not be opened");
    let mut archive = unwrap_or_err!(zip::ZipArchive::new(zipfile), "ziparchive could not be created");
    for i in 0..archive.len() {
        let mut file = unwrap_or_err!(archive.by_index(i), "no file in ziparchive");
        if file.is_dir() {
            unwrap_or_err!(std::fs::create_dir_all(format!("{}/{}", &output_path, file.name())), "dir could not be created");
            continue;
        }

        let outpath = match file.enclosed_name() {
            Some(r) => r.to_owned(),
            None => continue,
        };
        let mut outfile = unwrap_or_err!(std::fs::File::create(format!("{}/{}", output_path, outpath.to_str().unwrap())), "file could not be created");
        unwrap_or_err!(std::io::copy(&mut file, &mut outfile), "content could not be copied");
    }

    Ok(clienturls.shockwave_windows_version.parse::<usize>().unwrap())
}

#[tauri::command]
pub fn installed_version() -> Result<usize, &'static str> {
    let versions_path = std::path::Path::new("versions");
    if !versions_path.exists() || !versions_path.is_dir() {
        if std::fs::create_dir("versions").is_err() {
            return Err("");
        }
    }

    let mut installed_version = 0;
    'searchloop: for i in unwrap_or_err!(std::fs::read_dir("versions"), "dir could not be read") {
        let entry = match i {
            Ok(r) => r,
            Err(_) => continue 'searchloop
        };

        match entry.file_type() {
            Ok(r) => if !r.is_dir() { continue 'searchloop; },
            Err(_) => continue 'searchloop
        };
        
        if let Ok(r) = entry.file_name().to_str().unwrap().parse::<usize>() {
            if r > installed_version {
                installed_version = r;
            }
        }
    }

    Ok(installed_version)
}

#[tauri::command]
pub fn launch() -> Result<(), &'static str>{
    // TODO
    Ok(())
}

#[tauri::command]
pub fn user_lookup(username: &str) -> Result<String, &'static str> {
    let url = format!("{}{}", crate::LOOKUP_URL, username);
    let client = reqwest::blocking::Client::new();
    let res = unwrap_or_err!(
        client.get(url).header(reqwest::header::CONTENT_TYPE, "application/json").header(reqwest::header::USER_AGENT, "Mozilla/1.22 (compatible; MSIE 5.01; PalmOS 3.0) EudoraWeb 2").send(),
        "request failed"
    );
    
    let content = unwrap_or_err!(res.text(), "missing content");
    Ok(content)
}
