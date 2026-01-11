// FrictionalBank Dashboard Application

const API_BASE = '/api';
let ws = null;
let exposureChart = null;

// Format number with commas and decimals
function formatNumber(n, decimals = 2) {
    return n.toLocaleString('en-US', {
        minimumFractionDigits: decimals,
        maximumFractionDigits: decimals
    });
}

// Format currency
function formatCurrency(n) {
    const prefix = n >= 0 ? '' : '-';
    return prefix + '$' + formatNumber(Math.abs(n));
}

// Update element with value and color class
function updateValue(id, value, isPositive = null) {
    const el = document.getElementById(id);
    if (!el) return;

    el.textContent = formatCurrency(value);
    el.classList.remove('positive', 'negative');

    if (isPositive === true) el.classList.add('positive');
    else if (isPositive === false) el.classList.add('negative');
    else if (value > 0) el.classList.add('positive');
    else if (value < 0) el.classList.add('negative');
}

// Fetch portfolio data
async function fetchPortfolio() {
    try {
        const response = await fetch(`${API_BASE}/portfolio`);
        const data = await response.json();

        // Update dashboard
        updateValue('total-pv', data.total_pv);
        document.getElementById('trade-count').textContent = data.trade_count;

        // Update portfolio table
        const tbody = document.getElementById('portfolio-body');
        tbody.innerHTML = data.trades.map(t => `
            <tr>
                <td>${t.id}</td>
                <td>${t.instrument}</td>
                <td>${formatNumber(t.notional, 0)}</td>
                <td class="${t.pv >= 0 ? 'positive' : 'negative'}">${formatCurrency(t.pv)}</td>
                <td>${t.delta.toFixed(4)}</td>
                <td>${t.gamma.toFixed(4)}</td>
                <td>${t.vega.toFixed(4)}</td>
            </tr>
        `).join('');

    } catch (error) {
        console.error('Failed to fetch portfolio:', error);
    }
}

// Fetch risk metrics
async function fetchRiskMetrics() {
    try {
        const response = await fetch(`${API_BASE}/risk`);
        const data = await response.json();

        // Update dashboard
        updateValue('cva', data.cva);
        updateValue('dva', data.dva);
        updateValue('fva', data.fva);
        updateValue('total-xva', data.total_xva);

        // Update risk view
        updateValue('risk-cva', data.cva);
        updateValue('risk-dva', data.dva);
        updateValue('risk-fva', data.fva);
        updateValue('risk-total-xva', data.total_xva);
        document.getElementById('risk-ee').textContent = formatCurrency(data.ee);
        document.getElementById('risk-epe').textContent = formatCurrency(data.epe);
        document.getElementById('risk-pfe').textContent = formatCurrency(data.pfe);

    } catch (error) {
        console.error('Failed to fetch risk metrics:', error);
    }
}

// Fetch exposure data and update chart
async function fetchExposure() {
    try {
        const response = await fetch(`${API_BASE}/exposure`);
        const data = await response.json();

        updateExposureChart(data.time_series);

    } catch (error) {
        console.error('Failed to fetch exposure:', error);
    }
}

// Create or update exposure chart
function updateExposureChart(timeSeries) {
    const ctx = document.getElementById('exposure-chart');
    if (!ctx) return;

    const labels = timeSeries.map(p => p.time.toFixed(1) + 'Y');
    const pfeData = timeSeries.map(p => p.pfe);
    const eeData = timeSeries.map(p => p.ee);
    const epeData = timeSeries.map(p => p.epe);
    const eneData = timeSeries.map(p => p.ene);

    if (exposureChart) {
        // Update existing chart
        exposureChart.data.labels = labels;
        exposureChart.data.datasets[0].data = pfeData;
        exposureChart.data.datasets[1].data = eeData;
        exposureChart.data.datasets[2].data = epeData;
        exposureChart.data.datasets[3].data = eneData;
        exposureChart.update();
    } else {
        // Create new chart
        exposureChart = new Chart(ctx, {
            type: 'line',
            data: {
                labels: labels,
                datasets: [
                    {
                        label: 'PFE (95%)',
                        data: pfeData,
                        borderColor: '#f59e0b',
                        backgroundColor: 'rgba(245, 158, 11, 0.1)',
                        fill: false,
                        tension: 0.4
                    },
                    {
                        label: 'EE',
                        data: eeData,
                        borderColor: '#0ea5e9',
                        backgroundColor: 'rgba(14, 165, 233, 0.1)',
                        fill: false,
                        tension: 0.4
                    },
                    {
                        label: 'EPE',
                        data: epeData,
                        borderColor: '#22c55e',
                        backgroundColor: 'rgba(34, 197, 94, 0.1)',
                        fill: false,
                        tension: 0.4
                    },
                    {
                        label: 'ENE',
                        data: eneData,
                        borderColor: '#ef4444',
                        backgroundColor: 'rgba(239, 68, 68, 0.1)',
                        fill: false,
                        tension: 0.4
                    }
                ]
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                plugins: {
                    legend: {
                        display: false
                    }
                },
                scales: {
                    x: {
                        grid: {
                            color: '#334155'
                        },
                        ticks: {
                            color: '#94a3b8'
                        }
                    },
                    y: {
                        grid: {
                            color: '#334155'
                        },
                        ticks: {
                            color: '#94a3b8',
                            callback: function(value) {
                                if (Math.abs(value) >= 1000000) {
                                    return (value / 1000000).toFixed(1) + 'M';
                                } else if (Math.abs(value) >= 1000) {
                                    return (value / 1000).toFixed(0) + 'K';
                                }
                                return value;
                            }
                        }
                    }
                }
            }
        });
    }
}

// Connect to WebSocket
function connectWebSocket() {
    const statusEl = document.getElementById('connection-status');
    const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
    const wsUrl = `${protocol}//${window.location.host}${API_BASE}/ws`;

    ws = new WebSocket(wsUrl);

    ws.onopen = () => {
        statusEl.textContent = 'Connected';
        statusEl.classList.remove('error');
        statusEl.classList.add('connected');
    };

    ws.onclose = () => {
        statusEl.textContent = 'Disconnected';
        statusEl.classList.remove('connected');
        statusEl.classList.add('error');
        // Reconnect after 3 seconds
        setTimeout(connectWebSocket, 3000);
    };

    ws.onerror = () => {
        statusEl.textContent = 'Connection Error';
        statusEl.classList.remove('connected');
        statusEl.classList.add('error');
    };

    ws.onmessage = (event) => {
        try {
            const data = JSON.parse(event.data);
            handleWebSocketMessage(data);
        } catch (error) {
            console.error('Failed to parse WebSocket message:', error);
        }
    };
}

// Handle WebSocket messages
function handleWebSocketMessage(data) {
    if (data.type === 'connected') {
        console.log('WebSocket:', data.message);
    } else if (data.type === 'risk') {
        // Update risk metrics from real-time data
        const d = data.data;
        updateValue('total-pv', d.total_pv);
        updateValue('cva', d.cva);
        updateValue('dva', d.dva);
        updateValue('fva', d.fva);
    } else if (data.type === 'exposure') {
        // Update exposure metrics
        const d = data.data;
        document.getElementById('risk-ee').textContent = formatCurrency(d.ee);
        document.getElementById('risk-epe').textContent = formatCurrency(d.epe);
        document.getElementById('risk-pfe').textContent = formatCurrency(d.pfe);
    }
}

// Navigation
function setupNavigation() {
    const navBtns = document.querySelectorAll('.nav-btn');
    const views = document.querySelectorAll('.view');

    navBtns.forEach(btn => {
        btn.addEventListener('click', () => {
            const viewId = btn.dataset.view + '-view';

            // Update active states
            navBtns.forEach(b => b.classList.remove('active'));
            btn.classList.add('active');

            views.forEach(v => v.classList.remove('active'));
            document.getElementById(viewId).classList.add('active');

            // Fetch data for specific views
            if (btn.dataset.view === 'exposure') {
                fetchExposure();
            }
        });
    });
}

// Initialise application
async function init() {
    setupNavigation();

    // Initial data fetch
    await Promise.all([
        fetchPortfolio(),
        fetchRiskMetrics(),
        fetchExposure()
    ]);

    // Connect WebSocket
    connectWebSocket();

    // Periodic refresh (every 30 seconds)
    setInterval(() => {
        fetchPortfolio();
        fetchRiskMetrics();
    }, 30000);
}

// Start application when DOM is ready
document.addEventListener('DOMContentLoaded', init);
