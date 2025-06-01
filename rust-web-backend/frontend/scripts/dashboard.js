/**
 * 仪表板功能模块
 * 负责数据概览、图表展示和关键指标监控
 */
class DashboardModule {
    constructor(app) {
        this.app = app;
        this.charts = {};
        this.refreshInterval = null;
    }

    /**
     * 初始化仪表板
     */
    async init() {
        await this.loadDashboardData();
        this.setupCharts();
        this.startAutoRefresh();
    }

    /**
     * 加载仪表板数据
     */
    async loadDashboardData() {
        try {
            // 获取统计数据
            const stats = await this.app.api.getStatistics();
            this.renderStatistics(stats);

            // 获取排行榜数据
            const rankings = await this.app.api.getDoctorRankings();
            this.renderRankings(rankings);

            // 获取权重配置
            const weights = await this.app.api.getWeights();
            this.renderWeightsSummary(weights);

        } catch (error) {
            console.error('加载仪表板数据失败:', error);
            this.app.showNotification('加载数据失败，请稍后重试', 'error');
        }
    }

    /**
     * 渲染统计数据
     */
    renderStatistics(stats) {
        const statsContainer = document.getElementById('stats-overview');
        if (!statsContainer) return;

        statsContainer.innerHTML = `
            <div class="stats-grid">
                <div class="stat-card">
                    <div class="stat-value">${stats.total_doctors || 0}</div>
                    <div class="stat-label">医生总数</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">${this.app.formatNumber(stats.avg_influence_score)}</div>
                    <div class="stat-label">平均影响力</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">${this.app.formatNumber(stats.avg_quality_score)}</div>
                    <div class="stat-label">平均内容质量</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">${this.app.formatNumber(stats.avg_activity_score)}</div>
                    <div class="stat-label">平均活跃度</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">${this.app.formatNumber(stats.avg_total_score)}</div>
                    <div class="stat-label">平均总分</div>
                </div>
                <div class="stat-card">
                    <div class="stat-value">${stats.high_potential_count || 0}</div>
                    <div class="stat-label">高潜力医生</div>
                </div>
            </div>
        `;
    }

    /**
     * 渲染排行榜
     */
    renderRankings(rankings) {
        const rankingsContainer = document.getElementById('top-doctors');
        if (!rankingsContainer) return;

        const topDoctors = rankings.slice(0, 10);
        
        rankingsContainer.innerHTML = `
            <div class="rankings-header">
                <h3>医生排行榜 TOP 10</h3>
                <div class="ranking-filters">
                    <select id="ranking-type" class="form-select">
                        <option value="total">综合评分</option>
                        <option value="influence">影响力</option>
                        <option value="quality">内容质量</option>
                        <option value="activity">活跃度</option>
                    </select>
                </div>
            </div>
            <div class="rankings-list">
                ${topDoctors.map((doctor, index) => `
                    <div class="ranking-item">
                        <div class="ranking-position">${index + 1}</div>
                        <div class="doctor-info">
                            <div class="doctor-name">${doctor.name}</div>
                            <div class="doctor-title">${doctor.title}</div>
                            <div class="doctor-department">${doctor.department}</div>
                        </div>
                        <div class="doctor-scores">
                            <div class="score-item">
                                <span class="score-label">总分</span>
                                <span class="score-value">${this.app.formatNumber(doctor.total_score)}</span>
                            </div>
                            <div class="score-item">
                                <span class="score-label">影响力</span>
                                <span class="score-value">${this.app.formatNumber(doctor.influence_score)}</span>
                            </div>
                        </div>
                        <div class="investment-level ${this.getInvestmentClass(doctor.recommendation)}">
                            ${doctor.recommendation}
                        </div>
                    </div>
                `).join('')}
            </div>
        `;

        // 绑定排行榜类型切换事件
        const rankingTypeSelect = document.getElementById('ranking-type');
        if (rankingTypeSelect) {
            rankingTypeSelect.addEventListener('change', (e) => {
                this.updateRankings(e.target.value);
            });
        }
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
     * 更新排行榜
     */
    async updateRankings(type) {
        try {
            const rankings = await this.app.api.getDoctorRankings(type);
            this.renderRankings(rankings);
        } catch (error) {
            console.error('更新排行榜失败:', error);
        }
    }

    /**
     * 渲染权重配置摘要
     */
    renderWeightsSummary(weights) {
        const weightsContainer = document.getElementById('weights-summary');
        if (!weightsContainer) return;

        weightsContainer.innerHTML = `
            <div class="weights-overview">
                <h3>当前权重配置</h3>
                <div class="weights-grid">
                    <div class="weight-item">
                        <div class="weight-label">影响力权重</div>
                        <div class="weight-value">${(weights.influence_weight * 100).toFixed(1)}%</div>
                    </div>
                    <div class="weight-item">
                        <div class="weight-label">内容质量权重</div>
                        <div class="weight-value">${(weights.quality_weight * 100).toFixed(1)}%</div>
                    </div>
                    <div class="weight-item">
                        <div class="weight-label">活跃度权重</div>
                        <div class="weight-value">${(weights.activity_weight * 100).toFixed(1)}%</div>
                    </div>
                    <div class="weight-item">
                        <div class="weight-label">职称加权</div>
                        <div class="weight-value">${(weights.title_weight_factor * 100).toFixed(1)}%</div>
                    </div>
                </div>
                <div class="weights-actions">
                    <button class="btn btn-outline" onclick="app.navigate('weights')">
                        配置权重
                    </button>
                </div>
            </div>
        `;
    }

    /**
     * 设置图表
     */
    setupCharts() {
        this.setupScoreDistributionChart();
        this.setupTrendChart();
    }

    /**
     * 设置评分分布图表
     */
    async setupScoreDistributionChart() {
        const chartContainer = document.getElementById('score-distribution-chart');
        if (!chartContainer) return;

        try {
            const doctors = await this.app.api.getDoctors();
            
            // 计算评分分布
            const scoreRanges = [
                { range: '0-20', min: 0, max: 20, count: 0 },
                { range: '21-40', min: 21, max: 40, count: 0 },
                { range: '41-60', min: 41, max: 60, count: 0 },
                { range: '61-80', min: 61, max: 80, count: 0 },
                { range: '81-100', min: 81, max: 100, count: 0 }
            ];

            doctors.forEach(doctor => {
                const score = doctor.total_score || 0;
                for (let range of scoreRanges) {
                    if (score >= range.min && score <= range.max) {
                        range.count++;
                        break;
                    }
                }
            });

            // 渲染简单的条形图
            const maxCount = Math.max(...scoreRanges.map(r => r.count));
            
            chartContainer.innerHTML = `
                <div class="chart-title">评分分布</div>
                <div class="bar-chart">
                    ${scoreRanges.map(range => `
                        <div class="bar-item">
                            <div class="bar-label">${range.range}</div>
                            <div class="bar-container">
                                <div class="bar" style="width: ${maxCount > 0 ? (range.count / maxCount) * 100 : 0}%"></div>
                                <div class="bar-value">${range.count}</div>
                            </div>
                        </div>
                    `).join('')}
                </div>
            `;
        } catch (error) {
            console.error('设置评分分布图表失败:', error);
        }
    }

    /**
     * 设置趋势图表
     */
    setupTrendChart() {
        const chartContainer = document.getElementById('trend-chart');
        if (!chartContainer) return;

        // 模拟趋势数据
        const trendData = [
            { month: '1月', avgScore: 65.2 },
            { month: '2月', avgScore: 67.8 },
            { month: '3月', avgScore: 70.1 },
            { month: '4月', avgScore: 72.5 },
            { month: '5月', avgScore: 75.3 },
            { month: '6月', avgScore: 78.2 }
        ];

        const maxScore = Math.max(...trendData.map(d => d.avgScore));
        const minScore = Math.min(...trendData.map(d => d.avgScore));
        const scoreRange = maxScore - minScore;

        chartContainer.innerHTML = `
            <div class="chart-title">平均评分趋势</div>
            <div class="line-chart">
                <div class="chart-area">
                    ${trendData.map((data, index) => {
                        const height = scoreRange > 0 ? ((data.avgScore - minScore) / scoreRange) * 100 : 50;
                        return `
                            <div class="chart-point" style="left: ${(index / (trendData.length - 1)) * 100}%; bottom: ${height}%">
                                <div class="point-value">${data.avgScore.toFixed(1)}</div>
                            </div>
                        `;
                    }).join('')}
                </div>
                <div class="chart-labels">
                    ${trendData.map(data => `
                        <div class="chart-label">${data.month}</div>
                    `).join('')}
                </div>
            </div>
        `;
    }

    /**
     * 开始自动刷新
     */
    startAutoRefresh() {
        // 每5分钟自动刷新一次数据
        this.refreshInterval = setInterval(() => {
            this.loadDashboardData();
        }, 5 * 60 * 1000);
    }

    /**
     * 停止自动刷新
     */
    stopAutoRefresh() {
        if (this.refreshInterval) {
            clearInterval(this.refreshInterval);
            this.refreshInterval = null;
        }
    }

    /**
     * 销毁仪表板
     */
    destroy() {
        this.stopAutoRefresh();
        this.charts = {};
    }
}

// 导出模块
if (typeof module !== 'undefined' && module.exports) {
    module.exports = DashboardModule;
} else {
    window.DashboardModule = DashboardModule;
}
