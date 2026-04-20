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
    <div className="bg-gray-50 dark:bg-gray-800 rounded-lg p-6">
      <div className="flex justify-between items-center mb-4">
        <h3 className="text-lg font-semibold text-black dark:text-white">Active Downloads ({activeDownloads.length})</h3>
        {completedDownloads.length > 0 && (
          <button onClick={onClearCompleted} className="px-4 py-2 border border-gray-200 dark:border-gray-600 bg-gray-200 dark:bg-gray-700 text-black dark:text-white rounded-lg cursor-pointer text-xs font-medium transition-all hover:bg-black dark:hover:bg-white hover:text-white dark:hover:text-black">
            Clear Completed
          </button>
        )}
      </div>

      {activeDownloads.length === 0 && completedDownloads.length === 0 ? (
        <div className="text-center py-8 text-gray-400 text-sm">No downloads</div>
      ) : (
        <div className="flex flex-col gap-3">
          {downloads.map((download) => (
            <div key={download.id} className="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-700 rounded-lg p-4">
              <div className="flex items-center gap-3">
                <PlatformIcon platform={download.platform} size={20} />
                <div className="flex-1 min-w-0">
                  <div className="text-sm font-medium text-black dark:text-white overflow-hidden text-ellipsis whitespace-nowrap">{download.title}</div>
                  <div className={`text-xs font-medium mt-1 ${getStatusColor(download.status)}`}>
                    {download.status.charAt(0).toUpperCase() + download.status.slice(1)}
                  </div>
                </div>
                {(download.status === 'pending' || download.status === 'downloading') && (
                  <button
                    onClick={() => onCancel(download.id)}
                    className="px-4 py-2 border border-gray-200 dark:border-gray-600 bg-gray-200 dark:bg-gray-700 text-black dark:text-white rounded-lg cursor-pointer text-xs font-medium transition-all hover:bg-black dark:hover:bg-white hover:text-white dark:hover:text-black"
                  >
                    Cancel
                  </button>
                )}
              </div>
              
              {download.status === 'downloading' && (
                <div className="relative h-1.5 bg-gray-200 dark:bg-gray-600 rounded mt-3 overflow-hidden">
                  <div 
                    className="h-full bg-black dark:bg-white transition-all duration-300" 
                    style={{ width: `${download.progress}%` }}
                  />
                  <span className="absolute right-0 -top-5 text-xs font-medium text-gray-600 dark:text-gray-400">{Math.round(download.progress)}%</span>
                </div>
              )}
            </div>
          ))}
        </div>
      )}
    </div>
  );
};
