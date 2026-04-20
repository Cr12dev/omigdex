-- Example Plugin for Omigdex
-- This plugin demonstrates how to create a Lua plugin for Omigdex

-- Plugin initialization function (called when plugin is loaded)
function init()
    omigdex.log("Example plugin initialized!")
end

-- Event: Called when a download starts
-- data: JSON string containing download information
function on_download_start(data)
    omigdex.log("Download started: " .. data)
end

-- Event: Called when a download completes
-- data: JSON string containing download information
function on_download_complete(data)
    omigdex.log("Download completed: " .. data)
    
    -- Example: Write download info to a log file
    local log_content = "Download completed at: " .. os.date() .. "\n"
    omigdex.write_file("download_log.txt", log_content)
end

-- Event: Called when a download fails
-- data: JSON string containing download information including error
function on_download_error(data)
    omigdex.log("Download failed: " .. data)
end

-- Custom function that can be called from the application
function get_download_stats()
    local status = omigdex.get_queue_status()
    return string.format("Active: %d, Pending: %d, Total: %d",
        status.active_downloads,
        status.pending_downloads,
        status.total_downloads)
end

-- Example: Check video info before download
function validate_url(url)
    local info = omigdex.get_video_info(url)
    if info then
        omigdex.log("Video title: " .. info.title)
        omigdex.log("Platform: " .. info.platform)
        return true
    end
    return false
end
