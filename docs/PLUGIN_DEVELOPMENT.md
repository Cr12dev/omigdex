# Plugin Development Guide for Omigdex

This guide explains how to create Lua plugins for Omigdex, a video downloader application. Plugins allow you to extend the functionality of the application by hooking into download events and accessing backend functionality.

## Table of Contents

- [Overview](#overview)
- [Plugin Structure](#plugin-structure)
- [Available API Functions](#available-api-functions)
- [Event Hooks](#event-hooks)
- [Creating Your First Plugin](#creating-your-first-plugin)
- [Plugin Examples](#plugin-examples)
- [Best Practices](#best-practices)
- [Testing Plugins](#testing-plugins)

## Overview

Omigdex uses a Lua-based plugin system that runs on the backend (Rust). Plugins are written in Lua 5.4 and can:

- Hook into download events (start, complete, error)
- Access download queue and history
- Read and write files
- Log messages to the console
- Extend application functionality

Plugins are loaded from the `./plugins` directory in the application folder. Each plugin must be a `.lua` file.

## Plugin Structure

A basic plugin has the following structure:

```lua
-- Plugin initialization (optional)
function init()
    -- Called when the plugin is loaded
end

-- Event hooks (optional)
function on_download_start(data)
    -- Called when a download starts
end

function on_download_complete(data)
    -- Called when a download completes
end

function on_download_error(data)
    -- Called when a download fails
end

-- Custom functions (optional)
function my_custom_function()
    -- Your custom logic
end
```

## Available API Functions

The `omigdex` table provides the following functions:

### Logging

- `omigdex.log(message: string)` - Log a message to the console

  ```lua
  omigdex.log("Hello from plugin!")
  ```

### Download Information

- `omigdex.get_video_info(url: string) -> table` - Get video information from a URL

  ```lua
  local info = omigdex.get_video_info("https://youtube.com/watch?v=...")
  -- Returns: { title: string, platform: string }
  ```

### Queue Management

- `omigdex.get_queue_status() -> table` - Get current download queue status

  ```lua
  local status = omigdex.get_queue_status()
  -- Returns: { active_downloads: number, pending_downloads: number, total_downloads: number }
  ```

### History

- `omigdex.get_history() -> table` - Get download history

  ```lua
  local history = omigdex.get_history()
  -- Returns array of download entries
  for i, download in ipairs(history) do
      print(download.title)
      print(download.platform)
      print(download.format)
      print(download.status)
  end
  ```

### File Operations

- `omigdex.read_file(path: string) -> string` - Read file contents

  ```lua
  local content = omigdex.read_file("config.txt")
  ```

- `omigdex.write_file(path: string, content: string)` - Write content to a file

  ```lua
  omigdex.write_file("output.txt", "Hello, World!")
  ```

## Event Hooks

Plugins can hook into the following events:

### `on_download_start(data)`

Called when a download starts.

- **Parameters**: `data` (string) - JSON string containing download information
- **Example**:

  ```lua
  function on_download_start(data)
      omigdex.log("Download started: " .. data)
  end
  ```

### `on_download_complete(data)`

Called when a download completes successfully.

- **Parameters**: `data` (string) - JSON string containing download information
- **Example**:

  ```lua
  function on_download_complete(data)
      omigdex.log("Download completed: " .. data)
      -- Log to file
      omigdex.write_file("completed.log", data .. "\n", "a")
  end
  ```

### `on_download_error(data)`

Called when a download fails.

- **Parameters**: `data` (string) - JSON string containing download information including error details
- **Example**:

  ```lua
  function on_download_error(data)
      omigdex.log("Download failed: " .. data)
      -- Send notification or log error
  end
  ```

## Creating Your First Plugin

### Step 1: Create the Plugin File

Create a new `.lua` file in the `plugins` directory:

```bash
# In the application directory
mkdir plugins
# Create your plugin
echo "" > plugins/my_plugin.lua
```

### Step 2: Write Your Plugin

Edit the plugin file with your code:

```lua
-- my_plugin.lua

function init()
    omigdex.log("My plugin loaded!")
end

function on_download_complete(data)
    omigdex.log("Download finished successfully!")
    
    -- Example: Save download info to a custom log
    local log_entry = os.date("%Y-%m-%d %H:%M:%S") .. " - " .. data .. "\n"
    local current_log = omigdex.read_file("my_downloads.log") or ""
    omigdex.write_file("my_downloads.log", current_log .. log_entry)
end
```

### Step 3: Reload Plugins

Use the Tauri command to reload plugins:

```javascript
// From the frontend
await invoke('reload_plugins');
```

### Step 4: Enable Your Plugin

```javascript
// Enable the plugin
await invoke('enable_plugin', { name: 'my_plugin' });
```

## Plugin Examples

### Example 1: Download Logger

Logs all downloads to a file with timestamps:

```lua
-- download_logger.lua

function on_download_complete(data)
    local timestamp = os.date("%Y-%m-%d %H:%M:%S")
    local log_entry = "[" .. timestamp .. "] " .. data .. "\n"
    
    local existing_log = omigdex.read_file("downloads.log") or ""
    omigdex.write_file("downloads.log", existing_log .. log_entry)
    
    omigdex.log("Logged download to downloads.log")
end
```

### Example 2: Platform Statistics

Tracks download statistics by platform:

```lua
-- platform_stats.lua

local stats = {}

function init()
    -- Load existing stats
    local stats_file = omigdex.read_file("platform_stats.json") or "{}"
    stats = stats_file
    omigdex.log("Platform stats plugin initialized")
end

function on_download_complete(data)
    -- Parse the data (simplified - in real implementation use JSON library)
    -- For now, just log the completion
    omigdex.log("Download completed - update stats")
end
```

### Example 3: Download Validator

Validates URLs before download:

```lua
-- url_validator.lua

function validate_download(url)
    local info = omigdex.get_video_info(url)
    if info then
        omigdex.log("Valid URL - Title: " .. info.title)
        return true
    else
        omigdex.log("Invalid URL or video not found")
        return false
    end
end
```

## Best Practices

1. **Error Handling**: Always handle potential errors in your plugins

   ```lua
   local success, err = pcall(function()
       omigdex.write_file("output.txt", content)
   end)
   
   if not success then
       omigdex.log("Error writing file: " .. err)
   end
   ```

2. **Performance**: Keep plugin logic lightweight to avoid slowing down downloads

3. **Logging**: Use `omigdex.log()` for debugging and monitoring

4. **File Paths**: Use relative paths when possible for portability

5. **Naming**: Use descriptive plugin names (e.g., `download_logger.lua` instead of `plugin1.lua`)

6. **Documentation**: Comment your plugin code for maintainability

## Testing Plugins

### Manual Testing

1. Place your plugin in the `plugins` directory
2. Reload plugins using the Tauri command
3. Trigger a download and check console logs
4. Verify plugin behavior

### Debugging

Use `omigdex.log()` to debug your plugin:

```lua
function init()
    omigdex.log("Plugin initialization started")
    -- Your code
    omigdex.log("Plugin initialization completed")
end
```

### Common Issues

- **Plugin not loading**: Check file extension is `.lua`
- **Syntax errors**: Check Lua syntax using a Lua linter
- **Permission errors**: Ensure the application has write permissions to the plugin directory

## Plugin Management Commands

The following Tauri commands are available for plugin management:

- `get_plugins()` - Get list of loaded plugins
- `reload_plugins()` - Reload all plugins from the plugins directory
- `enable_plugin(name)` - Enable a specific plugin
- `disable_plugin(name)` - Disable a specific plugin

## Advanced Topics

### JSON Parsing

For complex data handling, you may need to parse JSON. Since Lua doesn't have built-in JSON support, you can:

1. Use a simple string manipulation for basic needs
2. Include a JSON library in your plugin (if supported in future versions)

### Async Operations

Currently, plugins run synchronously. Long-running operations should be kept to a minimum to avoid blocking the application.

## Support

For issues or questions about plugin development:
- Check the example plugin in `plugins/example_plugin.lua`
- Review the Omigdex documentation
- Open an issue on the GitHub repository
