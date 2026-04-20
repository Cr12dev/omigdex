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
    <div className="bg-gray-50 dark:bg-gray-800 rounded-lg p-6">
      <div className="flex justify-between items-center mb-4">
        <h3 className="text-lg font-semibold text-black dark:text-white">Download History ({history.length})</h3>
        {history.length > 0 && (
          <button onClick={handleClear} className="px-4 py-2 border border-gray-200 dark:border-gray-600 bg-gray-200 dark:bg-gray-700 text-black dark:text-white rounded-lg cursor-pointer text-xs font-medium transition-all hover:bg-black dark:hover:bg-white hover:text-white dark:hover:text-black">
            Clear All
          </button>
        )}
      </div>

      {history.length === 0 ? (
        <div className="text-center py-8 text-gray-400 text-sm">No download history</div>
      ) : (
        <div className="flex flex-col gap-3">
          {history.map((download) => (
            <div key={download.id} className="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-700 rounded-lg p-4">
              <div className="flex items-center gap-3">
                <PlatformIcon platform={download.platform} size={20} />
                <div className="flex-1 min-w-0">
                  <div className="text-sm font-medium text-black dark:text-white overflow-hidden text-ellipsis whitespace-nowrap">{download.title}</div>
                  <div className="text-xs text-gray-400 mt-1 flex items-center gap-2">
                    <span>{download.format.toUpperCase()}</span>
                    <span>•</span>
                    <span>{download.quality}</span>
                    <span>•</span>
                    <span>{formatDate(download.created_at)}</span>
                  </div>
                </div>
                <button
                  onClick={() => handleRemove(download.id)}
                  className="px-4 py-2 border border-gray-200 dark:border-gray-600 bg-gray-200 dark:bg-gray-700 text-black dark:text-white rounded-lg cursor-pointer text-xs font-medium transition-all hover:bg-black dark:hover:bg-white hover:text-white dark:hover:text-black"
                >
                  Remove
                </button>
              </div>
              
              {download.file_path && (
                <div className="text-xs text-gray-400 mt-2 break-all">{download.file_path}</div>
              )}
            </div>
          ))}
        </div>
      )}
    </div>
  );
};
