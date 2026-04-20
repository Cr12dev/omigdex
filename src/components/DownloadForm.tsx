import React, { useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface DownloadFormProps {
  onDownloadStart: (url: string, format: string, quality: string, gpuAcceleration: boolean) => void;
}

export const DownloadForm: React.FC<DownloadFormProps> = ({ onDownloadStart }) => {
  const [url, setUrl] = useState('');
  const [format, setFormat] = useState('mp4');
  const [quality, setQuality] = useState('high');
  const [gpuAcceleration, setGpuAcceleration] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState('');

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError('');

    if (!url.trim()) {
      setError('Please enter a URL');
      return;
    }

    setIsLoading(true);

    try {
      await invoke('check_yt_dlp');
      onDownloadStart(url, format, quality, gpuAcceleration);
      setUrl('');
    } catch (err) {
      setError('yt-dlp is not installed. Please install it from https://github.com/yt-dlp/yt-dlp');
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <form onSubmit={handleSubmit} className="mb-8">
      <div className="mb-4">
        <input
          type="text"
          value={url}
          onChange={(e) => setUrl(e.target.value)}
          placeholder="Paste video URL (YouTube, Instagram, TikTok, Pinterest)"
          className="w-full px-4 py-3 border border-gray-200 dark:border-gray-700 rounded-lg bg-gray-50 dark:bg-gray-800 text-black dark:text-white text-base font-inherit transition-colors focus:outline-none focus:border-black dark:focus:border-white placeholder-gray-400"
          disabled={isLoading}
        />
      </div>
      
      <div className="grid grid-cols-2 gap-4">
        <div className="mb-4">
          <label htmlFor="format" className="block mb-2 text-sm font-medium text-gray-600 dark:text-gray-400">Format</label>
          <select
            id="format"
            value={format}
            onChange={(e) => setFormat(e.target.value)}
            className="w-full px-4 py-3 border border-gray-200 dark:border-gray-700 rounded-lg bg-gray-50 dark:bg-gray-800 text-black dark:text-white text-base font-inherit transition-colors focus:outline-none focus:border-black dark:focus:border-white"
            disabled={isLoading}
          >
            <option value="mp4">MP4 (Video)</option>
            <option value="mp3">MP3 (Audio)</option>
          </select>
        </div>

        <div className="mb-4">
          <label htmlFor="quality" className="block mb-2 text-sm font-medium text-gray-600 dark:text-gray-400">Quality</label>
          <select
            id="quality"
            value={quality}
            onChange={(e) => setQuality(e.target.value)}
            className="w-full px-4 py-3 border border-gray-200 dark:border-gray-700 rounded-lg bg-gray-50 dark:bg-gray-800 text-black dark:text-white text-base font-inherit transition-colors focus:outline-none focus:border-black dark:focus:border-white"
            disabled={isLoading}
          >
            <option value="best">Best</option>
            <option value="high">High (1080p)</option>
            <option value="medium">Medium (720p)</option>
            <option value="low">Low</option>
          </select>
        </div>
      </div>

      <div className="mb-4">
        <label className="flex items-center space-x-2 cursor-pointer">
          <input
            type="checkbox"
            checked={gpuAcceleration}
            onChange={(e) => setGpuAcceleration(e.target.checked)}
            disabled={isLoading || format === 'mp3'}
            className="w-4 h-4 border border-gray-300 rounded bg-gray-50 dark:bg-gray-800 text-black dark:text-white focus:ring-black dark:focus:ring-white"
          />
          <span className="text-sm font-medium text-gray-600 dark:text-gray-400">
            GPU Acceleration (faster downloads, MP4 only)
          </span>
        </label>
      </div>

      {error && <div className="px-3 py-2 bg-red-500 text-white rounded-lg text-sm mb-4">{error}</div>}
      
      <button type="submit" className="w-full px-6 py-3 border-none rounded-lg bg-black dark:bg-white text-white dark:text-black text-base font-semibold cursor-pointer transition-colors hover:bg-gray-800 dark:hover:bg-gray-200 disabled:opacity-60 disabled:cursor-not-allowed" disabled={isLoading}>
        {isLoading ? 'Processing...' : 'Download'}
      </button>
    </form>
  );
};
