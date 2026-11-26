import React, { useState } from 'react';

export default function CommandPanel({ tools, onClose }) {
  const [filter, setFilter] = useState('');

  const filteredTools = tools.filter(t =>
    t.name.includes(filter) || t.desc.includes(filter)
  );

  return (
    <div style={{ padding: '20px', maxWidth: '600px', margin: '0 auto' }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <h2>命令面板</h2>
        <button onClick={onClose}>关闭</button>
      </div>
      <input
        type="text"
        placeholder="搜索工具..."
        value={filter}
        onChange={(e) => setFilter(e.target.value)}
        style={{ width: '100%', padding: '8px', margin: '16px 0' }}
      />
      <ul style={{ listStyle: 'none', padding: 0 }}>
        {filteredTools.map(tool => (
          <li key={tool.id} style={{ padding: '12px', border: '1px solid #ddd', marginBottom: '8px' }}>
            <strong>{tool.name}</strong>
            <div style={{ color: '#666', fontSize: '14px' }}>{tool.desc}</div>
          </li>
        ))}
      </ul>
    </div>
  );
}
