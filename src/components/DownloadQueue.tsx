import React from 'react';
import { PlatformIcon } from './PlatformIcon';

interface DownloadInfo {
  id: string;
  url: string;
  title: string;
  platform: string;
  format: string;
  quality: string;
  status: string;
  progress: number;
  file_path: string | null;
  created_at: string;
}

interface DownloadQueueProps {
  downloads: DownloadInfo[];
  onCancel: (id: string) => void;
  onClearCompleted: () => void;
}

export const DownloadQueue: React.FC<DownloadQueueProps> = ({ downloads, onCancel, onClearCompleted }) => {
  const getStatusColor = (status: string) => {
    switch (status.toLowerCase()) {
      case 'downloading': return 'text-blue-500';
      case 'completed': return 'text-green-500';
      case 'failed': return 'text-red-500';
      case 'cancelled': return 'text-gray-500';
      default: return 'text-gray-400';
    }
  };

  const activeDownloads = downloads.filter(d => d.status !== 'completed' && d.status !== 'failed' && d.status !== 'cancelled');
  const completedDownloads = downloads.filter(d => d.status === 'completed' || d.status === 'failed' || d.status === 'cancelled');

  return (
    <div className="download-queue">
      <div className="queue-header">
        <h3>Active Downloads ({activeDownloads.length})</h3>
        {completedDownloads.length > 0 && (
          <button onClick={onClearCompleted} className="clear-button">
            Clear Completed
          </button>
        )}
      </div>

      {activeDownloads.length === 0 && completedDownloads.length === 0 ? (
        <div className="empty-state">No downloads</div>
      ) : (
        <div className="queue-list">
          {downloads.map((download) => (
            <div key={download.id} className="queue-item">
              <div className="queue-item-header">
                <PlatformIcon platform={download.platform} size={20} />
                <div className="queue-item-info">
                  <div className="queue-item-title">{download.title}</div>
                  <div className={`queue-item-status ${getStatusColor(download.status)}`}>
                    {download.status.charAt(0).toUpperCase() + download.status.slice(1)}
                  </div>
                </div>
                {(download.status === 'pending' || download.status === 'downloading') && (
                  <button
                    onClick={() => onCancel(download.id)}
                    className="cancel-button"
                  >
                    Cancel
                  </button>
                )}
              </div>
              
              {download.status === 'downloading' && (
                <div className="progress-bar">
                  <div 
                    className="progress-fill" 
                    style={{ width: `${download.progress}%` }}
                  />
                  <span className="progress-text">{Math.round(download.progress)}%</span>
                </div>
              )}
            </div>
          ))}
        </div>
      )}
    </div>
  );
};
