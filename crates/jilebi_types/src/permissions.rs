use std::{borrow::Cow, collections::HashSet, path::Path};

use rustyscript::{PermissionCheckError, PermissionDeniedError, WebPermissions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JilebiPermissions {
    #[serde(default = "HashSet::new")]
    pub hosts: HashSet<String>,
    #[serde(default = "HashSet::new")]
    pub urls: HashSet<String>,
    #[serde(default = "HashSet::new")]
    pub read_files: HashSet<String>,
    #[serde(default = "HashSet::new")]
    pub write_files: HashSet<String>,
    #[serde(default = "HashSet::new")]
    pub read_dirs: HashSet<String>,
    #[serde(default = "HashSet::new")]
    pub write_dirs: HashSet<String>,
}

impl WebPermissions for JilebiPermissions {
    fn allow_hrtime(&self) -> bool {
        true
    }

    fn check_url(
        &self,
        url: &rustyscript::deno_core::url::Url,
        api_name: &str,
    ) -> Result<(), rustyscript::PermissionCheckError> {
        let url = String::from(url.as_str());

        let hostname_match = self
            .hosts
            .iter()
            .any(|host_name| url.starts_with(host_name));

        let url_match = self.urls.contains(&url);
        tracing::info!(
            "Checking URL permissions: url = {}, hostname_match={}, url_match={}",
            url,
            hostname_match,
            url_match
        );
        if hostname_match || url_match {
            Ok(())
        } else {
            Err(PermissionCheckError::PermissionDenied(
                PermissionDeniedError {
                    access: format!("NETWORK_FETCH_DENIED: {api_name}"),
                    name: "The url is not allowed to be accessed by your plugin. Declare these permissions in your plugin manifest",
                },
            ))
        }
    }

    fn check_open<'a>(
        &self,
        _resolved: bool,
        _read: bool,
        _write: bool,
        path: Cow<'a, Path>,
        _api_name: &str,
    ) -> Option<std::borrow::Cow<'a, std::path::Path>> {
        let path_str = path.to_str()?;
        if self.read_files.contains(path_str)
            || self.write_files.contains(path_str)
            || self.read_dirs.iter().any(|dir| path_str.starts_with(dir))
            || self.write_dirs.iter().any(|dir| path_str.starts_with(dir))
        {
            Some(path)
        } else {
            None
        }
    }

    fn check_read<'a>(
        &self,
        p: Cow<'a, Path>,
        api_name: Option<&str>,
    ) -> Result<std::borrow::Cow<'a, std::path::Path>, rustyscript::PermissionCheckError> {
        let path = String::from(p.to_str().unwrap_or_default());
        let api_name = api_name.unwrap_or_default();
        if self.read_files.contains(&path) || self.read_dirs.iter().any(|dir| path.starts_with(dir))
        {
            Ok(p)
        } else {
            Err(PermissionCheckError::PermissionDenied(
                PermissionDeniedError {
                    access: format!("FILE_READ_DENIED: {api_name}"),
                    name: "the file cannot be read because it was not declared in the plugin manifest under read_files, read_dirs",
                },
            ))
        }
    }

    fn check_read_all(
        &self,
        _api_name: Option<&str>,
    ) -> Result<(), rustyscript::PermissionCheckError> {
        Ok(())
    }

    fn check_read_blind(
        &self,
        p: &std::path::Path,
        _display: &str,
        api_name: &str,
    ) -> Result<(), rustyscript::PermissionCheckError> {
        self.check_read(Cow::Borrowed(p), Some(api_name))
            .map(|_| ())
    }

    fn check_write<'a>(
        &self,
        p: Cow<'a, Path>,
        api_name: Option<&str>,
    ) -> Result<std::borrow::Cow<'a, std::path::Path>, rustyscript::PermissionCheckError> {
        let path = String::from(p.to_str().unwrap_or_default());
        let api_name = api_name.unwrap_or_default();
        if self.write_files.contains(&path)
            || self.write_dirs.iter().any(|dir| path.starts_with(dir))
        {
            Ok(p)
        } else {
            Err(PermissionCheckError::PermissionDenied(
                PermissionDeniedError {
                    access: format!("FILE_WRITE_DENIED {}", api_name),
                    name: "the file cannot be written to since it has not been declared in the plugin manifest under write_files, write_dirs",
                },
            ))
        }
    }

    fn check_write_all(&self, _api_name: &str) -> Result<(), rustyscript::PermissionCheckError> {
        Ok(())
    }

    fn check_write_blind(
        &self,
        p: &std::path::Path,
        _display: &str,
        api_name: &str,
    ) -> Result<(), rustyscript::PermissionCheckError> {
        self.check_write(Cow::Borrowed(p), Some(api_name))
            .map(|_| ())
    }

    fn check_host(
        &self,
        host: &str,
        _port: Option<u16>,
        api_name: &str,
    ) -> Result<(), rustyscript::PermissionCheckError> {
        if self.hosts.contains(host) {
            Ok(())
        } else {
            Err(PermissionCheckError::PermissionDenied(
                PermissionDeniedError {
                    access: format!("NETWORK_FETCH_DENIED: {api_name}"),
                    name: "The url is not allowed to be accessed by your plugin. Declare these permissions in your plugin manifest under hosts",
                },
            ))
        }
    }

    fn check_sys(
        &self,
        _kind: rustyscript::SystemsPermissionKind,
        api_name: &str,
    ) -> Result<(), rustyscript::PermissionCheckError> {
        Err(PermissionCheckError::PermissionDenied(
            PermissionDeniedError {
                access: format!("SYS_OP_DENIED: {api_name}"),
                name: "system calls are not allowed",
            },
        ))
    }

    fn check_env(&self, var: &str) -> Result<(), rustyscript::PermissionCheckError> {
        Err(PermissionCheckError::PermissionDenied(
            PermissionDeniedError {
                access: format!("ENV_READ_DENIED {var}"),
                name: "Environment variables cannot be read",
            },
        ))
    }

    fn check_exec(&self) -> Result<(), rustyscript::PermissionCheckError> {
        Err(PermissionCheckError::PermissionDenied(
            PermissionDeniedError {
                access: "EXEC_DENIED".into(),
                name: "Execution commands are not allowed",
            },
        ))
    }

    fn check_vsock(
        &self,
        _cid: u32,
        _port: u32,
        _api_name: &str,
    ) -> Result<(), rustyscript::PermissionCheckError> {
        Err(PermissionCheckError::PermissionDenied(
            PermissionDeniedError {
                access: "VSOCK_DENIED".into(),
                name: "socket commands are not allowed",
            },
        ))
    }

    fn check_write_partial<'a>(
        &self,
        p: Cow<'a, Path>,
        api_name: &str,
    ) -> Result<Cow<'a, Path>, rustyscript::PermissionCheckError> {
        self.check_write(p, Some(api_name))
    }
}
