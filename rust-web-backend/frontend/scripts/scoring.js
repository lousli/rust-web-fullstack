/**
 * 评分系统功能模块
 * 负责评分计算、排名比较和评分趋势分析
 */
class ScoringModule {
    constructor(app) {
        this.app = app;
        this.currentScores = [];
        this.comparisonData = null;
        this.activeTab = 'rankings';
    }

    /**
     * 初始化评分模块
     */
    async init() {
        this.setupEventListeners();
        await this.loadScores();
        this.renderContent();
    }

    /**
     * 设置事件监听器
     */
    setupEventListeners() {
        // 标签页切换
        document.querySelectorAll('.tab-button').forEach(button => {
            button.addEventListener('click', (e) => {
                const tab = e.target.dataset.tab;
                this.switchTab(tab);
            });
        });

        // 重新计算评分按钮
        const recalculateBtn = document.getElementById('recalculate-scores-btn');
        if (recalculateBtn) {
            recalculateBtn.addEventListener('click', () => {
                this.recalculateScores();
            });
        }

        // 导出排名按钮
        const exportRankingBtn = document.getElementById('export-ranking-btn');
        if (exportRankingBtn) {
            exportRankingBtn.addEventListener('click', () => {
                this.exportRankings();
            });
        }

        // 评分筛选
        const scoreFilterSelect = document.getElementById('score-filter');
        if (scoreFilterSelect) {
            scoreFilterSelect.addEventListener('change', (e) => {
                this.filterByScore(e.target.value);
            });
        }

        // 投放建议筛选
        const recommendationFilter = document.getElementById('recommendation-filter');
        if (recommendationFilter) {
            recommendationFilter.addEventListener('change', (e) => {
                this.filterByRecommendation(e.target.value);
            });
        }
    }

    /**
     * 切换标签页
     */
    switchTab(tab) {
        this.activeTab = tab;
        
        // 更新标签按钮状态
        document.querySelectorAll('.tab-button').forEach(button => {
            button.classList.toggle('active', button.dataset.tab === tab);
        });

        // 更新内容区域
        this.renderContent();
    }

    /**
     * 加载评分数据
     */
    async loadScores() {
        try {
            this.app.showLoading(true);
            
            // 获取医生评分排名
            this.currentScores = await this.app.api.getDoctorRankings();
            
            // 获取比较数据（模拟历史数据）
            this.comparisonData = await this.generateComparisonData();
            
        } catch (error) {
            console.error('加载评分数据失败:', error);
            this.app.showNotification('加载评分数据失败', 'error');
            this.currentScores = [];
        } finally {
            this.app.showLoading(false);
        }
    }

    /**
     * 生成比较数据（模拟）
     */
    async generateComparisonData() {
        // 模拟历史数据对比
        return this.currentScores.map(doctor => ({
            ...doctor,
            previous_total_score: doctor.total_score + (Math.random() - 0.5) * 10,
            previous_influence_score: doctor.influence_score + (Math.random() - 0.5) * 5,
            previous_quality_score: doctor.quality_score + (Math.random() - 0.5) * 5,
            previous_activity_score: doctor.activity_score + (Math.random() - 0.5) * 5
        }));
    }

    /**
     * 渲染内容
     */
    renderContent() {
        const contentContainer = document.getElementById('scoring-content');
        if (!contentContainer) return;

        switch (this.activeTab) {
            case 'rankings':
                this.renderRankings();
                break;
            case 'comparison':
                this.renderComparison();
                break;
            case 'trends':
                this.renderTrends();
                break;
            case 'analysis':
                this.renderAnalysis();
                break;
            default:
                this.renderRankings();
        }
    }

    /**
     * 渲染排名页面
     */
    renderRankings() {
        const contentContainer = document.getElementById('scoring-content');
        
        contentContainer.innerHTML = `
            <div class="rankings-section">
                <div class="section-header">
                    <h3>医生评分排名</h3>
                    <div class="section-actions">
                        <select id="score-filter" class="form-select">
                            <option value="">全部评分</option>
                            <option value="high">高分医生 (≥80)</option>
                            <option value="medium">中等医生 (60-79)</option>
                            <option value="low">低分医生 (<60)</option>
                        </select>
                        <select id="recommendation-filter" class="form-select">
                            <option value="">全部建议</option>
                            <option value="重点投放">重点投放</option>
                            <option value="适度投放">适度投放</option>
                            <option value="观察投放">观察投放</option>
                        </select>
                        <button id="recalculate-scores-btn" class="btn btn-primary">
                            重新计算评分
                        </button>
                        <button id="export-ranking-btn" class="btn btn-outline">
                            导出排名
                        </button>
                    </div>
                </div>
                <div class="rankings-grid" id="rankings-grid">
                    ${this.renderRankingCards()}
                </div>
            </div>
        `;

        this.bindRankingEvents();
    }

    /**
     * 渲染排名卡片
     */
    renderRankingCards() {
        if (this.currentScores.length === 0) {
            return `
                <div class="empty-state">
                    <div class="empty-icon">📊</div>
                    <div class="empty-title">暂无评分数据</div>
                    <div class="empty-description">请先添加医生数据并计算评分</div>
                </div>
            `;
        }

        return this.currentScores.map((doctor, index) => `
            <div class="ranking-card" data-doctor-id="${doctor.id}">
                <div class="ranking-header">
                    <div class="ranking-position">
                        <span class="rank-number">${index + 1}</span>
                        ${index < 3 ? this.getRankMedal(index) : ''}
                    </div>
                    <div class="doctor-basic">
                        <div class="doctor-name">${doctor.name}</div>
                        <div class="doctor-title">${doctor.title}</div>
                        <div class="doctor-hospital">${doctor.hospital}</div>
                    </div>
                </div>
                
                <div class="score-section">
                    <div class="total-score">
                        <span class="score-value">${this.app.formatNumber(doctor.total_score)}</span>
                        <span class="score-label">总分</span>
                    </div>
                    
                    <div class="score-breakdown">
                        <div class="score-item">
                            <div class="score-bar">
                                <div class="score-fill" style="width: ${doctor.influence_score}%"></div>
                            </div>
                            <div class="score-info">
                                <span class="score-name">影响力</span>
                                <span class="score-num">${this.app.formatNumber(doctor.influence_score)}</span>
                            </div>
                        </div>
                        
                        <div class="score-item">
                            <div class="score-bar">
                                <div class="score-fill" style="width: ${doctor.quality_score}%"></div>
                            </div>
                            <div class="score-info">
                                <span class="score-name">内容质量</span>
                                <span class="score-num">${this.app.formatNumber(doctor.quality_score)}</span>
                            </div>
                        </div>
                        
                        <div class="score-item">
                            <div class="score-bar">
                                <div class="score-fill" style="width: ${doctor.activity_score}%"></div>
                            </div>
                            <div class="score-info">
                                <span class="score-name">活跃度</span>
                                <span class="score-num">${this.app.formatNumber(doctor.activity_score)}</span>
                            </div>
                        </div>
                    </div>
                </div>
                
                <div class="recommendation-section">
                    <div class="investment-recommendation ${this.getInvestmentClass(doctor.recommendation)}">
                        ${doctor.recommendation || '待评估'}
                    </div>
                    <div class="card-actions">
                        <button class="btn btn-sm btn-outline" onclick="scoringModule.viewDoctorDetail(${doctor.id})">
                            详情
                        </button>
                        <button class="btn btn-sm btn-outline" onclick="scoringModule.compareDoctors([${doctor.id}])">
                            对比
                        </button>
                    </div>
                </div>
            </div>
        `).join('');
    }

    /**
     * 获取排名奖章
     */
    getRankMedal(index) {
        const medals = ['🥇', '🥈', '🥉'];
        return `<span class="rank-medal">${medals[index]}</span>`;
    }

    /**
     * 获取投放建议样式类名
     */
    getInvestmentClass(recommendation) {
        switch (recommendation) {
            case '重点投放': return 'investment-high';
            case '适度投放': return 'investment-medium';
            case '观察投放': return 'investment-low';
            default: return 'investment-none';
        }
    }

    /**
     * 绑定排名事件
     */
    bindRankingEvents() {
        // 重新绑定筛选事件
        const scoreFilter = document.getElementById('score-filter');
        if (scoreFilter) {
            scoreFilter.addEventListener('change', (e) => {
                this.filterByScore(e.target.value);
            });
        }

        const recommendationFilter = document.getElementById('recommendation-filter');
        if (recommendationFilter) {
            recommendationFilter.addEventListener('change', (e) => {
                this.filterByRecommendation(e.target.value);
            });
        }

        const recalculateBtn = document.getElementById('recalculate-scores-btn');
        if (recalculateBtn) {
            recalculateBtn.addEventListener('click', () => {
                this.recalculateScores();
            });
        }

        const exportBtn = document.getElementById('export-ranking-btn');
        if (exportBtn) {
            exportBtn.addEventListener('click', () => {
                this.exportRankings();
            });
        }
    }

    /**
     * 渲染对比页面
     */
    renderComparison() {
        const contentContainer = document.getElementById('scoring-content');
        
        contentContainer.innerHTML = `
            <div class="comparison-section">
                <div class="section-header">
                    <h3>医生对比分析</h3>
                    <div class="section-actions">
                        <button class="btn btn-primary" onclick="scoringModule.showComparisonSelector()">
                            选择对比医生
                        </button>
                    </div>
                </div>
                <div id="comparison-content">
                    ${this.renderComparisonPlaceholder()}
                </div>
            </div>
        `;
    }

    /**
     * 渲染对比占位符
     */
    renderComparisonPlaceholder() {
        return `
            <div class="comparison-placeholder">
                <div class="placeholder-icon">⚖️</div>
                <div class="placeholder-title">选择医生进行对比分析</div>
                <div class="placeholder-description">
                    选择2-5位医生进行多维度对比分析，了解各项指标的差异
                </div>
                <button class="btn btn-primary" onclick="scoringModule.showComparisonSelector()">
                    开始对比
                </button>
            </div>
        `;
    }

    /**
     * 渲染趋势分析页面
     */
    renderTrends() {
        const contentContainer = document.getElementById('scoring-content');
        
        contentContainer.innerHTML = `
            <div class="trends-section">
                <div class="section-header">
                    <h3>评分趋势分析</h3>
                    <div class="section-actions">
                        <select class="form-select">
                            <option value="7">最近7天</option>
                            <option value="30" selected>最近30天</option>
                            <option value="90">最近90天</option>
                        </select>
                    </div>
                </div>
                
                <div class="trends-grid">
                    <div class="trend-card">
                        <h4>平均评分趋势</h4>
                        <div class="trend-chart" id="average-score-trend">
                            ${this.renderTrendChart('average')}
                        </div>
                    </div>
                    
                    <div class="trend-card">
                        <h4>高分医生数量趋势</h4>
                        <div class="trend-chart" id="high-score-trend">
                            ${this.renderTrendChart('count')}
                        </div>
                    </div>
                    
                    <div class="trend-card">
                        <h4>各维度评分对比</h4>
                        <div class="dimension-comparison">
                            ${this.renderDimensionComparison()}
                        </div>
                    </div>
                    
                    <div class="trend-card">
                        <h4>投放建议分布</h4>
                        <div class="recommendation-distribution">
                            ${this.renderRecommendationDistribution()}
                        </div>
                    </div>
                </div>
            </div>
        `;
    }

    /**
     * 渲染趋势图表
     */
    renderTrendChart(type) {
        // 模拟趋势数据
        const data = Array.from({ length: 30 }, (_, i) => ({
            date: new Date(Date.now() - (29 - i) * 24 * 60 * 60 * 1000).toLocaleDateString(),
            value: 65 + Math.random() * 20 + Math.sin(i / 5) * 5
        }));

        const maxValue = Math.max(...data.map(d => d.value));
        const minValue = Math.min(...data.map(d => d.value));
        const range = maxValue - minValue;

        return `
            <div class="simple-line-chart">
                <div class="chart-area">
                    ${data.map((point, index) => {
                        const height = range > 0 ? ((point.value - minValue) / range) * 100 : 50;
                        const left = (index / (data.length - 1)) * 100;
                        return `
                            <div class="chart-point" 
                                 style="left: ${left}%; bottom: ${height}%" 
                                 title="${point.date}: ${point.value.toFixed(1)}">
                            </div>
                        `;
                    }).join('')}
                    <svg class="chart-line" viewBox="0 0 100 100" preserveAspectRatio="none">
                        <polyline 
                            points="${data.map((point, index) => {
                                const x = (index / (data.length - 1)) * 100;
                                const y = range > 0 ? 100 - ((point.value - minValue) / range) * 100 : 50;
                                return `${x},${y}`;
                            }).join(' ')}"
                            fill="none" 
                            stroke="var(--primary-color)" 
                            stroke-width="0.5"
                        />
                    </svg>
                </div>
                <div class="chart-labels">
                    <span>30天前</span>
                    <span>今天</span>
                </div>
            </div>
        `;
    }

    /**
     * 渲染维度对比
     */
    renderDimensionComparison() {
        const dimensions = [
            { name: '影响力', value: 72.5, color: '#3b82f6' },
            { name: '内容质量', value: 68.2, color: '#10b981' },
            { name: '活跃度', value: 75.8, color: '#f59e0b' }
        ];

        return `
            <div class="dimension-bars">
                ${dimensions.map(dim => `
                    <div class="dimension-bar">
                        <div class="bar-label">${dim.name}</div>
                        <div class="bar-container">
                            <div class="bar-fill" style="width: ${dim.value}%; background-color: ${dim.color}"></div>
                            <div class="bar-value">${dim.value.toFixed(1)}</div>
                        </div>
                    </div>
                `).join('')}
            </div>
        `;
    }

    /**
     * 渲染投放建议分布
     */
    renderRecommendationDistribution() {
        const distribution = [
            { label: '重点投放', count: 12, percentage: 20, color: '#ef4444' },
            { label: '适度投放', count: 28, percentage: 47, color: '#f59e0b' },
            { label: '观察投放', count: 20, percentage: 33, color: '#10b981' }
        ];

        return `
            <div class="distribution-chart">
                ${distribution.map(item => `
                    <div class="distribution-item">
                        <div class="item-color" style="background-color: ${item.color}"></div>
                        <div class="item-info">
                            <div class="item-label">${item.label}</div>
                            <div class="item-stats">
                                <span class="item-count">${item.count}人</span>
                                <span class="item-percentage">${item.percentage}%</span>
                            </div>
                        </div>
                    </div>
                `).join('')}
            </div>
        `;
    }

    /**
     * 渲染分析页面
     */
    renderAnalysis() {
        const contentContainer = document.getElementById('scoring-content');
        
        contentContainer.innerHTML = `
            <div class="analysis-section">
                <div class="section-header">
                    <h3>深度分析报告</h3>
                    <div class="section-actions">
                        <button class="btn btn-outline">生成报告</button>
                        <button class="btn btn-primary">导出分析</button>
                    </div>
                </div>
                
                <div class="analysis-grid">
                    <div class="analysis-card">
                        <h4>评分分布分析</h4>
                        <div class="analysis-content">
                            ${this.renderScoreDistributionAnalysis()}
                        </div>
                    </div>
                    
                    <div class="analysis-card">
                        <h4>投放建议分析</h4>
                        <div class="analysis-content">
                            ${this.renderInvestmentAnalysis()}
                        </div>
                    </div>
                    
                    <div class="analysis-card">
                        <h4>科室表现分析</h4>
                        <div class="analysis-content">
                            ${this.renderDepartmentAnalysis()}
                        </div>
                    </div>
                    
                    <div class="analysis-card">
                        <h4>职称影响分析</h4>
                        <div class="analysis-content">
                            ${this.renderTitleAnalysis()}
                        </div>
                    </div>
                </div>
            </div>
        `;
    }

    /**
     * 渲染评分分布分析
     */
    renderScoreDistributionAnalysis() {
        return `
            <div class="analysis-insights">
                <div class="insight-item">
                    <div class="insight-icon">📈</div>
                    <div class="insight-text">
                        平均评分为 <strong>72.5分</strong>，整体表现良好
                    </div>
                </div>
                <div class="insight-item">
                    <div class="insight-icon">⭐</div>
                    <div class="insight-text">
                        <strong>20%</strong> 的医生评分超过80分，具备重点投放潜力
                    </div>
                </div>
                <div class="insight-item">
                    <div class="insight-icon">📊</div>
                    <div class="insight-text">
                        评分集中在 <strong>60-80分</strong> 区间，分布相对均匀
                    </div>
                </div>
            </div>
        `;
    }

    /**
     * 渲染投放建议分析
     */
    renderInvestmentAnalysis() {
        return `
            <div class="analysis-insights">
                <div class="insight-item">
                    <div class="insight-icon">🎯</div>
                    <div class="insight-text">
                        建议重点投放医生 <strong>12人</strong>，预期ROI较高
                    </div>
                </div>
                <div class="insight-item">
                    <div class="insight-icon">💡</div>
                    <div class="insight-text">
                        适度投放医生 <strong>28人</strong>，可作为中期培养对象
                    </div>
                </div>
                <div class="insight-item">
                    <div class="insight-icon">👀</div>
                    <div class="insight-text">
                        观察投放医生 <strong>20人</strong>，需要持续关注表现
                    </div>
                </div>
            </div>
        `;
    }

    /**
     * 渲染科室表现分析
     */
    renderDepartmentAnalysis() {
        return `
            <div class="department-performance">
                <div class="performance-item">
                    <div class="dept-name">心内科</div>
                    <div class="dept-score">82.3</div>
                    <div class="dept-trend">📈</div>
                </div>
                <div class="performance-item">
                    <div class="dept-name">神经内科</div>
                    <div class="dept-score">78.9</div>
                    <div class="dept-trend">📈</div>
                </div>
                <div class="performance-item">
                    <div class="dept-name">消化内科</div>
                    <div class="dept-score">75.6</div>
                    <div class="dept-trend">📊</div>
                </div>
                <div class="performance-item">
                    <div class="dept-name">呼吸内科</div>
                    <div class="dept-score">71.2</div>
                    <div class="dept-trend">📉</div>
                </div>
            </div>
        `;
    }

    /**
     * 渲染职称影响分析
     */
    renderTitleAnalysis() {
        return `
            <div class="title-impact">
                <div class="impact-item">
                    <div class="title-name">主任医师</div>
                    <div class="impact-score">+15%</div>
                    <div class="impact-desc">权威性强，影响力显著</div>
                </div>
                <div class="impact-item">
                    <div class="title-name">副主任医师</div>
                    <div class="impact-score">+10%</div>
                    <div class="impact-desc">专业能力突出</div>
                </div>
                <div class="impact-item">
                    <div class="title-name">主治医师</div>
                    <div class="impact-score">+5%</div>
                    <div class="impact-desc">临床经验丰富</div>
                </div>
                <div class="impact-item">
                    <div class="title-name">住院医师</div>
                    <div class="impact-score">0%</div>
                    <div class="impact-desc">基础评分</div>
                </div>
            </div>
        `;
    }

    /**
     * 按评分筛选
     */
    filterByScore(scoreRange) {
        let filteredScores = [...this.currentScores];
        
        switch (scoreRange) {
            case 'high':
                filteredScores = filteredScores.filter(doctor => doctor.total_score >= 80);
                break;
            case 'medium':
                filteredScores = filteredScores.filter(doctor => doctor.total_score >= 60 && doctor.total_score < 80);
                break;
            case 'low':
                filteredScores = filteredScores.filter(doctor => doctor.total_score < 60);
                break;
        }

        this.renderFilteredRankings(filteredScores);
    }

    /**
     * 按投放建议筛选
     */
    filterByRecommendation(recommendation) {
        let filteredScores = [...this.currentScores];
        
        if (recommendation) {
            filteredScores = filteredScores.filter(doctor => doctor.recommendation === recommendation);
        }

        this.renderFilteredRankings(filteredScores);
    }

    /**
     * 渲染筛选后的排名
     */
    renderFilteredRankings(filteredScores) {
        const rankingsGrid = document.getElementById('rankings-grid');
        if (!rankingsGrid) return;

        const originalScores = this.currentScores;
        this.currentScores = filteredScores;
        rankingsGrid.innerHTML = this.renderRankingCards();
        this.currentScores = originalScores;
    }

    /**
     * 重新计算评分
     */
    async recalculateScores() {
        if (!confirm('确定要重新计算所有医生的评分吗？这可能需要一些时间。')) {
            return;
        }

        try {
            this.app.showLoading(true);
            await this.app.api.recalculateScores();
            await this.loadScores();
            this.renderContent();
            this.app.showNotification('评分重新计算完成', 'success');
        } catch (error) {
            console.error('重新计算评分失败:', error);
            this.app.showNotification('重新计算评分失败', 'error');
        } finally {
            this.app.showLoading(false);
        }
    }

    /**
     * 导出排名
     */
    async exportRankings() {
        try {
            const csvContent = this.generateRankingCSV();
            this.downloadCSV(csvContent, 'doctor_rankings.csv');
            this.app.showNotification('排名导出成功', 'success');
        } catch (error) {
            console.error('导出排名失败:', error);
            this.app.showNotification('导出排名失败', 'error');
        }
    }

    /**
     * 生成排名CSV
     */
    generateRankingCSV() {
        const headers = ['排名', '姓名', '职称', '医院', '科室', '总分', '影响力评分', '内容质量评分', '活跃度评分', '投放建议'];
        const rows = this.currentScores.map((doctor, index) => [
            index + 1,
            doctor.name,
            doctor.title,
            doctor.hospital,
            doctor.department,
            doctor.total_score.toFixed(2),
            doctor.influence_score.toFixed(2),
            doctor.quality_score.toFixed(2),
            doctor.activity_score.toFixed(2),
            doctor.recommendation || '待评估'
        ]);

        return [headers, ...rows].map(row => row.join(',')).join('\n');
    }

    /**
     * 下载CSV文件
     */
    downloadCSV(content, filename) {
        const blob = new Blob(['\ufeff' + content], { type: 'text/csv;charset=utf-8;' });
        const link = document.createElement('a');
        link.href = URL.createObjectURL(blob);
        link.download = filename;
        link.click();
        URL.revokeObjectURL(link.href);
    }

    /**
     * 查看医生详情
     */
    async viewDoctorDetail(doctorId) {
        // 切换到医生管理页面并显示详情
        this.app.navigate('doctors');
        // 延迟执行以确保页面已切换
        setTimeout(() => {
            if (window.doctorsModule) {
                window.doctorsModule.viewDoctor(doctorId);
            }
        }, 100);
    }

    /**
     * 对比医生
     */
    compareDoctors(doctorIds) {
        // 实现医生对比功能
        this.switchTab('comparison');
        // TODO: 实现具体的对比逻辑
    }

    /**
     * 显示对比选择器
     */
    showComparisonSelector() {
        // TODO: 实现对比医生选择器
        this.app.showNotification('对比功能开发中...', 'info');
    }
}

// 导出模块
if (typeof module !== 'undefined' && module.exports) {
    module.exports = ScoringModule;
} else {
    window.ScoringModule = ScoringModule;
}
