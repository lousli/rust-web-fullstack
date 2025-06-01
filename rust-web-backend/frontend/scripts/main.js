/**
 * 医生数据分析系统 - 主JavaScript文件
 * 处理前端交互逻辑和API调用
 */

class DoctorAnalysisSystem {
    constructor() {
        this.apiBase = '/api';
        this.currentPage = 1;
        this.pageSize = 10;
        this.currentSearch = '';
        this.doctors = [];
        this.scores = [];        this.weights = {
            performance_weight: 40,
            cost_performance_weight: 30,
            affinity_weight: 30
        };
        
        this.init();
    }

    /**
     * 初始化系统
     */
    init() {
        this.bindEvents();
        this.loadInitialData();
    }

    /**
     * 绑定事件监听器
     */
    bindEvents() {
        // 导航标签页切换
        document.querySelectorAll('.nav-tab').forEach(tab => {
            tab.addEventListener('click', (e) => {
                this.switchTab(e.target.dataset.tab);
            });
        });

        // 搜索功能
        const searchInput = document.getElementById('search-input');
        if (searchInput) {
            searchInput.addEventListener('input', this.debounce((e) => {
                this.currentSearch = e.target.value;
                this.currentPage = 1;
                this.loadDoctors();
            }, 300));
        }

        // 刷新按钮
        const refreshBtn = document.getElementById('refresh-btn');
        if (refreshBtn) {
            refreshBtn.addEventListener('click', () => {
                this.loadDoctors();
            });
        }

        // 分页按钮
        const prevBtn = document.getElementById('prev-page');
        const nextBtn = document.getElementById('next-page');
        if (prevBtn) {
            prevBtn.addEventListener('click', () => {
                if (this.currentPage > 1) {
                    this.currentPage--;
                    this.loadDoctors();
                }
            });
        }
        if (nextBtn) {
            nextBtn.addEventListener('click', () => {
                this.currentPage++;
                this.loadDoctors();
            });
        }

        // 权重配置
        this.bindWeightEvents();

        // 分析类型切换
        const analysisType = document.getElementById('analysis-type');
        if (analysisType) {
            analysisType.addEventListener('change', (e) => {
                this.loadAnalysis(e.target.value);
            });
        }

        // 错误提示关闭
        const closeError = document.getElementById('close-error');
        if (closeError) {
            closeError.addEventListener('click', () => {
                this.hideError();
            });
        }
    }

    /**
     * 绑定权重配置相关事件
     */
    bindWeightEvents() {
        const weightInputs = ['performance', 'cost', 'satisfaction'];
        
        weightInputs.forEach(type => {
            const input = document.getElementById(`${type}-weight`);
            const slider = document.getElementById(`${type}-slider`);
            
            if (input && slider) {
                // 输入框变化同步到滑块
                input.addEventListener('input', (e) => {
                    const value = parseInt(e.target.value) || 0;
                    slider.value = value;
                    this.updateWeights();
                });
                
                // 滑块变化同步到输入框
                slider.addEventListener('input', (e) => {
                    const value = parseInt(e.target.value);
                    input.value = value;
                    this.updateWeights();
                });
            }
        });

        // 保存权重配置
        const saveBtn = document.getElementById('save-weights');
        if (saveBtn) {
            saveBtn.addEventListener('click', () => {
                this.saveWeights();
            });
        }
    }

    /**
     * 切换标签页
     */
    switchTab(tabName) {
        // 更新导航标签样式
        document.querySelectorAll('.nav-tab').forEach(tab => {
            tab.classList.remove('active');
        });
        document.querySelector(`[data-tab="${tabName}"]`).classList.add('active');

        // 切换面板显示
        document.querySelectorAll('.panel').forEach(panel => {
            panel.classList.remove('active');
        });
        document.getElementById(`${tabName}-panel`).classList.add('active');

        // 加载对应数据
        switch (tabName) {
            case 'doctors':
                this.loadDoctors();
                break;
            case 'analysis':
                this.loadAnalysis();
                break;
            case 'weights':
                this.loadWeights();
                break;
        }
    }

    /**
     * 加载初始数据
     */
    async loadInitialData() {
        await this.loadDoctors();
    }    /**
     * 加载医生数据
     */
    async loadDoctors() {
        this.showLoading();
        
        try {
            const params = new URLSearchParams({
                page: this.currentPage,
                page_size: this.pageSize
            });
            
            if (this.currentSearch) {
                params.append('search', this.currentSearch);
            }

            const response = await fetch(`${this.apiBase}/doctors?${params}`);
            const data = await this.handleResponse(response);
            
            this.doctors = data.data || [];
            this.renderDoctorsTable();
            this.updatePagination(data.total || this.doctors.length);
            
        } catch (error) {
            this.showError('加载医生数据失败: ' + error.message);
        } finally {
            this.hideLoading();
        }
    }    /**
     * 渲染医生表格
     */
    renderDoctorsTable() {
        const tbody = document.getElementById('doctors-tbody');
        if (!tbody) return;

        if (this.doctors.length === 0) {
            tbody.innerHTML = `
                <tr>
                    <td colspan="7" style="text-align: center; padding: 2rem; color: #64748b;">
                        <i class="fas fa-search"></i><br>
                        暂无数据
                    </td>
                </tr>
            `;
            return;
        }

        tbody.innerHTML = this.doctors.map(doctor => `
            <tr>
                <td>${doctor.id}</td>
                <td><strong>${doctor.name}</strong></td>
                <td><span class="badge">${doctor.department}</span></td>
                <td>${doctor.title}</td>
                <td>${doctor.region}</td>
                <td>${this.formatDate(doctor.created_at)}</td>
                <td>
                    <button class="btn btn-primary btn-sm" onclick="system.viewDoctorDetails('${doctor.id}')">
                        <i class="fas fa-eye"></i> 查看
                    </button>
                </td>
            </tr>
        `).join('');
    }

    /**
     * 更新分页信息
     */
    updatePagination(total) {
        const totalPages = Math.ceil(total / this.pageSize);
        const pageInfo = document.getElementById('page-info');
        const prevBtn = document.getElementById('prev-page');
        const nextBtn = document.getElementById('next-page');

        if (pageInfo) {
            pageInfo.textContent = `第 ${this.currentPage} 页，共 ${totalPages} 页`;
        }

        if (prevBtn) {
            prevBtn.disabled = this.currentPage <= 1;
        }

        if (nextBtn) {
            nextBtn.disabled = this.currentPage >= totalPages;
        }
    }

    /**
     * 加载分析数据
     */
    async loadAnalysis(type = 'overview') {
        this.showLoading();
        
        try {
            // 加载统计数据
            const statsResponse = await fetch(`${this.apiBase}/scores/statistics`);
            const statsData = await this.handleResponse(statsResponse);
            this.renderStatistics(statsData);

            // 加载评分数据
            const scoresResponse = await fetch(`${this.apiBase}/scores`);
            const scoresData = await this.handleResponse(scoresResponse);
            this.scores = scoresData.data || [];
            this.renderScoresTable();
            
        } catch (error) {
            this.showError('加载分析数据失败: ' + error.message);
        } finally {
            this.hideLoading();
        }
    }

    /**
     * 渲染统计数据
     */
    renderStatistics(stats) {
        const elements = {
            'total-doctors': stats.total_doctors || 0,
            'avg-score': (stats.average_score || 0).toFixed(1),
            'total-departments': stats.unique_departments || 0,
            'top-performer': stats.top_performer || '-'
        };

        Object.entries(elements).forEach(([id, value]) => {
            const element = document.getElementById(id);
            if (element) {
                element.textContent = value;
            }
        });
    }    /**
     * 渲染评分表格
     */
    renderScoresTable() {
        const tbody = document.getElementById('scores-tbody');
        if (!tbody) return;

        if (this.scores.length === 0) {
            tbody.innerHTML = `
                <tr>
                    <td colspan="6" style="text-align: center; padding: 2rem; color: #64748b;">
                        <i class="fas fa-chart-bar"></i><br>
                        暂无评分数据
                    </td>
                </tr>
            `;
            return;
        }

        // 计算排名
        const sortedScores = [...this.scores].sort((a, b) => b.weighted_total_score - a.weighted_total_score);
        
        tbody.innerHTML = sortedScores.map((score, index) => `
            <tr>
                <td><strong>${score.doctor_name}</strong></td>
                <td>${score.performance_score.toFixed(1)}</td>
                <td>${score.cost_performance_score.toFixed(1)}</td>
                <td>${score.affinity_score.toFixed(1)}</td>
                <td><strong>${score.weighted_total_score.toFixed(1)}</strong></td>
                <td>
                    <span class="rank-badge rank-${this.getRankClass(index + 1)}">
                        #${index + 1}
                    </span>
                </td>
            </tr>
        `).join('');
    }

    /**
     * 获取排名样式类
     */
    getRankClass(rank) {
        if (rank === 1) return 'gold';
        if (rank === 2) return 'silver';
        if (rank === 3) return 'bronze';
        return 'default';
    }

    /**
     * 加载权重配置
     */
    async loadWeights() {
        try {
            const response = await fetch(`${this.apiBase}/weight-configs/active`);
            const data = await this.handleResponse(response);
            
            if (data.data) {
                this.weights = data.data;
                this.updateWeightsUI();
            }
        } catch (error) {
            console.warn('加载权重配置失败，使用默认值:', error.message);
        }
    }    /**
     * 更新权重UI
     */
    updateWeightsUI() {
        const mappings = {
            'performance': this.weights.performance_weight,
            'cost': this.weights.cost_performance_weight,
            'satisfaction': this.weights.affinity_weight
        };

        Object.entries(mappings).forEach(([type, value]) => {
            const input = document.getElementById(`${type}-weight`);
            const slider = document.getElementById(`${type}-slider`);
            
            if (input) input.value = value;
            if (slider) slider.value = value;
        });

        this.updateWeights();
    }

    /**
     * 更新权重计算
     */
    updateWeights() {
        const performance = parseInt(document.getElementById('performance-weight')?.value) || 0;
        const cost = parseInt(document.getElementById('cost-weight')?.value) || 0;
        const satisfaction = parseInt(document.getElementById('satisfaction-weight')?.value) || 0;
        
        const total = performance + cost + satisfaction;
        const totalElement = document.getElementById('total-weight');
        const statusElement = document.getElementById('weight-status');
        
        if (totalElement) {
            totalElement.textContent = total;
        }
        
        if (statusElement) {
            if (total === 100) {
                statusElement.textContent = '✓ 权重配置正常';
                statusElement.className = 'weight-status valid';
            } else {
                statusElement.textContent = '⚠ 权重总和必须为100%';
                statusElement.className = 'weight-status invalid';
            }
        }        // 更新权重对象
        this.weights = {
            performance_weight: performance,
            cost_performance_weight: cost,
            affinity_weight: satisfaction
        };
    }

    /**
     * 保存权重配置
     */
    async saveWeights() {
        const total = Object.values(this.weights).reduce((sum, val) => sum + val, 0);
        
        if (total !== 100) {
            this.showError('权重总和必须为100%');
            return;
        }

        this.showLoading();
        
        try {
            const response = await fetch(`${this.apiBase}/weight-configs`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },                body: JSON.stringify({
                    config_name: `权重配置_${new Date().toLocaleString()}`,
                    ...this.weights,
                    data_index_weight: 0.0,
                    editing_weight: 0.0,
                    video_quality_weight: 0.0,
                    is_active: true
                })
            });
            
            await this.handleResponse(response);
            this.showSuccess('权重配置保存成功！');
            
        } catch (error) {
            this.showError('保存权重配置失败: ' + error.message);
        } finally {
            this.hideLoading();
        }
    }

    /**
     * 查看医生详情
     */
    async viewDoctorDetails(doctorId) {
        try {
            const response = await fetch(`${this.apiBase}/doctors/${doctorId}`);
            const data = await this.handleResponse(response);
            
            // 这里可以实现详情弹窗或跳转到详情页
            alert(`医生详情：\n姓名: ${data.data.name}\n科室: ${data.data.department}\n职称: ${data.data.title}`);
            
        } catch (error) {
            this.showError('获取医生详情失败: ' + error.message);
        }
    }

    /**
     * 处理API响应
     */
    async handleResponse(response) {
        if (!response.ok) {
            const errorData = await response.json().catch(() => ({}));
            throw new Error(errorData.message || `HTTP ${response.status}: ${response.statusText}`);
        }
        return await response.json();
    }

    /**
     * 显示加载状态
     */
    showLoading() {
        const loading = document.getElementById('loading');
        if (loading) {
            loading.classList.remove('hidden');
        }
    }

    /**
     * 隐藏加载状态
     */
    hideLoading() {
        const loading = document.getElementById('loading');
        if (loading) {
            loading.classList.add('hidden');
        }
    }

    /**
     * 显示错误信息
     */
    showError(message) {
        const errorElement = document.getElementById('error-message');
        const errorText = document.getElementById('error-text');
        
        if (errorElement && errorText) {
            errorText.textContent = message;
            errorElement.classList.remove('hidden');
            
            // 3秒后自动隐藏
            setTimeout(() => {
                this.hideError();
            }, 5000);
        }
    }

    /**
     * 显示成功信息
     */
    showSuccess(message) {
        // 简单的成功提示，可以扩展为独立的成功提示组件
        const errorElement = document.getElementById('error-message');
        const errorText = document.getElementById('error-text');
        
        if (errorElement && errorText) {
            errorText.textContent = message;
            errorElement.style.background = 'var(--success-color)';
            errorElement.classList.remove('hidden');
            
            setTimeout(() => {
                this.hideError();
                errorElement.style.background = 'var(--danger-color)';
            }, 3000);
        }
    }

    /**
     * 隐藏错误信息
     */
    hideError() {
        const errorElement = document.getElementById('error-message');
        if (errorElement) {
            errorElement.classList.add('hidden');
        }
    }

    /**
     * 格式化日期
     */
    formatDate(dateString) {
        const date = new Date(dateString);
        return date.toLocaleDateString('zh-CN');
    }

    /**
     * 防抖函数
     */
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
}

// 添加一些CSS样式到文档中
const additionalStyles = `
<style>
.badge {
    background: var(--primary-color);
    color: white;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-size: 0.8rem;
}

.btn-sm {
    padding: 0.25rem 0.5rem;
    font-size: 0.8rem;
}

.rank-badge {
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-weight: bold;
    font-size: 0.8rem;
}

.rank-gold { background: #ffd700; color: #000; }
.rank-silver { background: #c0c0c0; color: #000; }
.rank-bronze { background: #cd7f32; color: #fff; }
.rank-default { background: var(--medium-gray); color: var(--dark-gray); }
</style>
`;

document.head.insertAdjacentHTML('beforeend', additionalStyles);

// 页面加载完成后初始化系统
let system;
document.addEventListener('DOMContentLoaded', () => {
    system = new DoctorAnalysisSystem();
});

// 导出系统实例供全局使用
window.system = system;
