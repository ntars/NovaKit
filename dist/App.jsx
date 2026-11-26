import React, { useState } from 'react';
import CommandPanel from './components/CommandPanel.jsx';
import Settings from './components/Settings.jsx';

export default function App() {
  const [view, setView] = useState('home');
  const [showCommandPanel, setShowCommandPanel] = useState(false);

  const tools = [
    { id: 'text.format', name: '文本格式化', desc: 'JSON/XML/代码格式化' },
    { id: 'text.encode', name: '编码转换', desc: 'UTF-8/GBK 等编码' },
    { id: 'text.compare', name: '文本比对', desc: '两文件差异对比' },
    { id: 'file.list', name: '文件列表', desc: '快速浏览目录' },
    { id: 'file.search', name: '文件搜索', desc: '按内容搜索' },
    { id: 'image.resize', name: '图片缩放', desc: '批量压缩图片' },
  ];

  const renderView = () => {
    if (showCommandPanel) {
      return <CommandPanel tools={tools} onClose={() => setShowCommandPanel(false)} />;
    }
    if (view === 'settings') {
      return <Settings onClose={() => setView('home')} />;
    }
    return (
      <div style={{ padding: '40px' }}>
        <h1>NovaKit</h1>
        <p style={{ color: '#666' }}>装完系统，只装这一套。</p>
        <button onClick={() => setShowCommandPanel(true)}>打开命令面板</button>
        <button onClick={() => setView('settings')}>设置</button>
      </div>
    );
  };

  return <>{renderView()}</>;
}
