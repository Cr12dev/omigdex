import React from 'react';
import { PlatformIcon } from './PlatformIcon';
import { invoke } from '@tauri-apps/api/core';

interface DownloadInfo {
  id: string;
  url: string;
  title: string;
  platform: string;
  format: string;
  quality: string;
  status: string;
  file_path: string | null;
  created_at: string;
}

interface DownloadHistoryProps {
  history: DownloadInfo[];
  onRefresh: () => void;
}

export const DownloadHistory: React.FC<DownloadHistoryProps> = ({ history, onRefresh }) => {
  const handleRemove = async (id: string) => {
    try {
      await invoke('remove_from_history', { id });
      onRefresh();
    } catch (error) {
      console.error('Failed to remove from history:', error);
    }
  };

  const handleClear = async () => {
    try {
      await invoke('clear_history');
      onRefresh();
    } catch (error) {
      console.error('Failed to clear history:', error);
    }
  };

  const formatDate = (dateString: string) => {
    const date = new Date(dateString);
    return date.toLocaleDateString() + ' ' + date.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
  };

  return (
    <div className="download-history">
      <div className="history-header">
        <h3>Download History ({history.length})</h3>
        {history.length > 0 && (
          <button onClick={handleClear} className="clear-button">
            Clear All
          </button>
        )}
      </div>

      {history.length === 0 ? (
        <div className="empty-state">No download history</div>
      ) : (
        <div className="history-list">
          {history.map((download) => (
            <div key={download.id} className="history-item">
              <div className="history-item-header">
                <PlatformIcon platform={download.platform} size={20} />
                <div className="history-item-info">
                  <div className="history-item-title">{download.title}</div>
                  <div className="history-item-meta">
                    <span>{download.format.toUpperCase()}</span>
                    <span>•</span>
                    <span>{download.quality}</span>
                    <span>•</span>
                    <span>{formatDate(download.created_at)}</span>
                  </div>
                </div>
                <button
                  onClick={() => handleRemove(download.id)}
                  className="remove-button"
                >
                  Remove
                </button>
              </div>
              
              {download.file_path && (
                <div className="history-item-path">{download.file_path}</div>
              )}
            </div>
          ))}
        </div>
      )}
    </div>
  );
};
