// ============================================================
// 硬件设备检测器 — 仪表盘
// ============================================================

const $ = id => document.getElementById(id);
const esc = t => { const e = document.createElement('div'); e.appendChild(document.createTextNode(t)); return e.innerHTML; };

let scanTimer = null;

function nowStr() {
  const d = new Date();
  return `${d.getFullYear()}-${String(d.getMonth()+1).padStart(2,'0')}-${String(d.getDate()).padStart(2,'0')} ${String(d.getHours()).padStart(2,'0')}:${String(d.getMinutes()).padStart(2,'0')}:${String(d.getSeconds()).padStart(2,'0')}`;
}

function ledClass(status) {
  switch (status) {
    case '正常': return 'ok';
    case '异常': case 'Error': return 'err';
    case '已断开': return 'disconnected';
    default: return 'unknown';
  }
}

function badgeClass(source) {
  const s = (source || '').toLowerCase();
  if (s.includes('usb')) return 'badge-usb';
  if (s.includes('bluetooth')) return 'badge-bluetooth';
  if (s.includes('wmi')) return 'badge-wmi';
  return 'badge-other';
}

async function scan() {
  const btn = $('scanBtn');
  const loading = $('loading');
  const empty = $('empty');
  const grid = $('device-grid');

  btn.disabled = true;
  btn.textContent = '检测中...';
  loading.style.display = 'block';
  empty.style.display = 'none';

  try {
    const result = await window.__TAURI__.core.invoke('detect_devices');
    const devices = result.devices || [];

    // 更新统计
    let usbN = 0, btN = 0, wmiN = 0;
    devices.forEach(d => {
      const s = (d.source || '').toLowerCase();
      if (s.includes('usb')) usbN++;
      else if (s.includes('bluetooth')) btN++;
      else wmiN++;
    });

    $('totalCount').textContent = result.total_count;
    $('usbCount').textContent = usbN;
    $('btCount').textContent = btN;
    $('wmiCount').textContent = wmiN;
    $('lastScan').textContent = nowStr();

    // 渲染
    if (devices.length === 0) {
      empty.style.display = 'block';
      empty.querySelector('p').textContent = '未检测到设备';
      grid.innerHTML = '';
      return;
    }

    empty.style.display = 'none';
    grid.innerHTML = devices.map(d => {
      const vendorHtml = d.vendor_name && d.vendor_name !== '未知厂商'
        ? `<span class="vendor-name">${esc(d.vendor_name)}</span>`
        : '';
      return `<div class="device-card">
        <span class="device-led ${ledClass(d.status)}"></span>
        <div class="device-body">
          <div class="device-name">${vendorHtml ? vendorHtml + ' ' : ''}${esc(d.description)}</div>
          <div class="device-meta">
            <span class="badge ${badgeClass(d.source)}">${esc(d.source)}</span>
            <span>${esc(d.class)}</span>
            <span class="device-vid">${esc(d.vendor_id)}:${esc(d.product_id)}</span>
          </div>
        </div>
      </div>`;
    }).join('');

  } catch (err) {
    console.error(err);
    empty.style.display = 'block';
    empty.querySelector('p').textContent = '检测失败: ' + esc(String(err));
  } finally {
    btn.disabled = false;
    btn.textContent = '开始检测';
    loading.style.display = 'none';
  }
}

// ============================================================
// 自动检测
// ============================================================

$('autoScan').addEventListener('change', function() {
  if (this.checked) {
    scan();
    scanTimer = setInterval(scan, 5000);
  } else {
    if (scanTimer) { clearInterval(scanTimer); scanTimer = null; }
  }
});

// ============================================================
// 键盘快捷键: Ctrl+Enter / Enter 触发检测
// ============================================================

document.addEventListener('keydown', e => {
  if ((e.key === 'Enter' && e.ctrlKey) || (e.key === 'F5')) {
    e.preventDefault();
    if (!$('scanBtn').disabled) scan();
  }
});
