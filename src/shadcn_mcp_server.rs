use zed_extension_api as zed;

struct ShadcnMcpExtension;

/// Resolves the `npx` binary that sits next to the given `node` binary.
fn npx_path_for(node_path: &str, platform: zed::Os) -> String {
    let npx = match platform {
        zed::Os::Windows => "npx.cmd",
        zed::Os::Mac | zed::Os::Linux => "npx",
    };
    match node_path.rfind(['/', '\\']) {
        Some(idx) => format!("{}{}", &node_path[..=idx], npx),
        None => npx.to_string(),
    }
}

impl zed::Extension for ShadcnMcpExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &zed::ContextServerId,
        _project: &zed::Project,
    ) -> zed::Result<zed::Command> {
        // Get the system's node binary path, npx should be in the same directory
        let node_path = zed::node_binary_path()?;
        let (platform, _) = zed::current_platform();
        let npx_path = npx_path_for(&node_path, platform);

        Ok(zed::Command {
            command: npx_path,
            args: vec!["shadcn@latest".to_string(), "mcp".to_string()],
            env: std::env::vars().collect(),
        })
    }
}

zed::register_extension!(ShadcnMcpExtension);
