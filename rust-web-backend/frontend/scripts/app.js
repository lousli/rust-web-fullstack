// 医生投放分析系统 - 主应用脚本
class DoctorAnalysisApp {
    constructor() {
        this.currentView = 'dashboard';
        this.apiBase = '/api';
        this.init();
    }

    async init() {
        this.setupEventListeners();
        this.setupNavigation();
        await this.loadInitialData();
    }

    setupEventListeners() {
        // 导航切换
        document.querySelectorAll('.nav-item').forEach(item => {
            item.addEventListener('click', (e) => {
                e.preventDefault();
                const view = item.dataset.view;
                this.switchView(view);
            });
        });

        // 刷新按钮
        document.getElementById('refresh-btn')?.addEventListener('click', () => {
            this.refreshCurrentView();
        });

        // 模态框关闭
        document.getElementById('modal-close')?.addEventListener('click', () => {
            this.hideModal();
        });

        document.getElementById('modal-overlay')?.addEventListener('click', (e) => {
            if (e.target.id === 'modal-overlay') {
                this.hideModal();
            }
        });
    }

    setupNavigation() {
        // 设置页面标题映射
        this.pageTitles = {
            dashboard: '数据面板',
            doctors: '医生管理',
            scoring: '评分分析',
            weights: '权重配置',
            import: '数据导入',
            reports: '分析报告'
        };
    }

    switchView(viewName) {
        // 更新导航状态
        document.querySelectorAll('.nav-item').forEach(item => {
            item.classList.remove('active');
        });
        document.querySelector(`[data-view="${viewName}"]`)?.classList.add('active');

        // 更新视图内容
        document.querySelectorAll('.view-content').forEach(view => {
            view.classList.remove('active');
        });
        document.getElementById(`${viewName}-view`)?.classList.add('active');

        // 更新页面标题
        const titleElement = document.getElementById('page-title');
        if (titleElement) {
            titleElement.textContent = this.pageTitles[viewName] || viewName;
        }

        this.currentView = viewName;
        this.loadViewData(viewName);
    }

    async loadViewData(viewName) {
        try {
            switch (viewName) {
                case 'dashboard':
                    await window.dashboard?.loadDashboard();
                    break;
                case 'doctors':
                    await window.doctors?.loadDoctors();
                    break;
                case 'scoring':
                    await window.scoring?.loadScoring();
                    break;
                case 'weights':
                    await window.weights?.loadWeights();
                    break;
                case 'import':
                    await window.importModule?.loadImport();
                    break;
                case 'reports':
                    await window.reports?.loadReports();
                    break;
            }
        } catch (error) {
            console.error(`加载视图 ${viewName} 时出错:`, error);
            this.showToast('加载数据时出错', 'error');
        }
    }

    async loadInitialData() {
        try {
            this.showLoading();
            await this.loadViewData('dashboard');
        } catch (error) {
            console.error('加载初始数据时出错:', error);
            this.showToast('初始化失败', 'error');
        } finally {
            this.hideLoading();
        }
    }

    async refreshCurrentView() {
        try {
            this.showLoading();
            await this.loadViewData(this.currentView);
            this.showToast('数据已刷新', 'success');
        } catch (error) {
            console.error('刷新数据时出错:', error);
            this.showToast('刷新失败', 'error');
        } finally {
            this.hideLoading();
        }
    }

    // API 请求方法
    async apiRequest(endpoint, options = {}) {
        const url = `${this.apiBase}${endpoint}`;
        const defaultOptions = {
            headers: {
                'Content-Type': 'application/json',
            },
        };

        const requestOptions = { ...defaultOptions, ...options };

        try {
            const response = await fetch(url, requestOptions);
            const data = await response.json();

            if (!response.ok) {
                throw new Error(data.message || `HTTP ${response.status}`);
            }

            return data;
        } catch (error) {
            console.error(`API 请求失败 ${endpoint}:`, error);
            throw error;
        }
    }

    // GET 请求
    async get(endpoint) {
        return this.apiRequest(endpoint, { method: 'GET' });
    }

    // POST 请求
    async post(endpoint, data) {
        return this.apiRequest(endpoint, {
            method: 'POST',
            body: JSON.stringify(data),
        });
    }

    // PUT 请求
    async put(endpoint, data) {
        return this.apiRequest(endpoint, {
            method: 'PUT',
            body: JSON.stringify(data),
        });
    }

    // DELETE 请求
    async delete(endpoint) {
        return this.apiRequest(endpoint, { method: 'DELETE' });
    }

    // 文件上传
    async uploadFile(endpoint, file) {
        const formData = new FormData();
        formData.append('file', file);

        return this.apiRequest(endpoint, {
            method: 'POST',
            body: formData,
            headers: {}, // 让浏览器自动设置 Content-Type
        });
    }

    // UI 工具方法
    showLoading() {
        document.getElementById('loading-indicator')?.classList.add('active');
    }

    hideLoading() {
        document.getElementById('loading-indicator')?.classList.remove('active');
    }

    showModal(title, content, footer = '') {
        const modal = document.getElementById('modal-overlay');
        const titleEl = document.getElementById('modal-title');
        const bodyEl = document.getElementById('modal-body');
        const footerEl = document.getElementById('modal-footer');

        if (titleEl) titleEl.textContent = title;
        if (bodyEl) bodyEl.innerHTML = content;
        if (footerEl) footerEl.innerHTML = footer;

        modal?.classList.add('active');
    }

    hideModal() {
        document.getElementById('modal-overlay')?.classList.remove('active');
    }

    showToast(message, type = 'info', duration = 3000) {
        const container = document.getElementById('toast-container');
        if (!container) return;

        const toast = document.createElement('div');
        toast.className = `toast ${type}`;
        
        const icon = this.getToastIcon(type);
        toast.innerHTML = `
            <i class="${icon}"></i>
            <span>${message}</span>
        `;

        container.appendChild(toast);

        // 自动移除
        setTimeout(() => {
            toast.remove();
        }, duration);
    }

    getToastIcon(type) {
        const icons = {
            success: 'fas fa-check-circle',
            error: 'fas fa-exclamation-circle',
            warning: 'fas fa-exclamation-triangle',
            info: 'fas fa-info-circle'
        };
        return icons[type] || icons.info;
    }

    // 格式化工具方法
    formatNumber(num, decimals = 0) {
        if (num === null || num === undefined) return '--';
        return Number(num).toLocaleString('zh-CN', {
            minimumFractionDigits: decimals,
            maximumFractionDigits: decimals
        });
    }

    formatPercent(num, decimals = 1) {
        if (num === null || num === undefined) return '--';
        return (Number(num) * 100).toFixed(decimals) + '%';
    }

    formatDate(dateString) {
        if (!dateString) return '--';
        return new Date(dateString).toLocaleString('zh-CN');
    }

    // 评分徽章
    getScoreBadge(score) {
        if (score >= 80) return '<span class="score-badge high">优秀</span>';
        if (score >= 60) return '<span class="score-badge medium">良好</span>';
        return '<span class="score-badge low">待改进</span>';
    }

    // 投放建议徽章
    getRecommendationBadge(recommendation) {
        const badges = {
            'highly_recommended': '<span class="recommendation-badge highly-recommended">强烈推荐</span>',
            'recommended': '<span class="recommendation-badge recommended">推荐</span>',
            'conditional': '<span class="recommendation-badge conditional">条件推荐</span>',
            'not_recommended': '<span class="recommendation-badge not-recommended">不推荐</span>'
        };
        return badges[recommendation] || '<span class="recommendation-badge">未知</span>';
    }

    // 数据验证
    validateRequired(value, fieldName) {
        if (!value || value.toString().trim() === '') {
            throw new Error(`${fieldName}不能为空`);
        }
    }

    validateNumber(value, fieldName, min = null, max = null) {
        const num = Number(value);
        if (isNaN(num)) {
            throw new Error(`${fieldName}必须是有效数字`);
        }
        if (min !== null && num < min) {
            throw new Error(`${fieldName}不能小于${min}`);
        }
        if (max !== null && num > max) {
            throw new Error(`${fieldName}不能大于${max}`);
        }
        return num;
    }

    validateEmail(email) {
        const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        if (!emailRegex.test(email)) {
            throw new Error('邮箱格式不正确');
        }
    }

    // 权限检查（预留）
    checkPermission(action) {
        // 这里可以实现权限检查逻辑
        return true;
    }

    // 导出CSV
    exportToCSV(data, filename) {
        if (!data || data.length === 0) {
            this.showToast('没有数据可导出', 'warning');
            return;
        }

        const headers = Object.keys(data[0]);
        const csvContent = [
            headers.join(','),
            ...data.map(row => headers.map(header => `"${row[header] || ''}"`).join(','))
        ].join('\n');

        const blob = new Blob([csvContent], { type: 'text/csv;charset=utf-8;' });
        const link = document.createElement('a');
        const url = URL.createObjectURL(blob);
        
        link.setAttribute('href', url);
        link.setAttribute('download', filename);
        link.style.visibility = 'hidden';
        
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);
    }

    // 防抖函数
    debounce(func, wait) {
        let timeout;
        return function executedFunction(...args) {
            const later = () => {
                clearTimeout(timeout);
                func(...args);
            };
            clearTimeout(timeout);
            timeout = setTimeout(later, wait);
        };
    }

    // 节流函数
    throttle(func, limit) {
        let inThrottle;
        return function(...args) {
            if (!inThrottle) {
                func.apply(this, args);
                inThrottle = true;
                setTimeout(() => inThrottle = false, limit);
            }
        };
    }
}

// 初始化应用
document.addEventListener('DOMContentLoaded', () => {
    window.app = new DoctorAnalysisApp();
});

// 全局错误处理
window.addEventListener('error', (event) => {
    console.error('全局错误:', event.error);
    if (window.app) {
        window.app.showToast('系统出现错误，请刷新页面重试', 'error');
    }
});

// 网络错误处理
window.addEventListener('online', () => {
    if (window.app) {
        window.app.showToast('网络连接已恢复', 'success');
    }
});

window.addEventListener('offline', () => {
    if (window.app) {
        window.app.showToast('网络连接已断开', 'warning');
    }
});
