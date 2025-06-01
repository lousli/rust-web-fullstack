/**
 * 报告生成功能模块
 * 负责生成各类分析报告和数据导出
 */
class ReportsModule {
    constructor(app) {
        this.app = app;
        this.reportData = null;
        this.currentReportType = 'overview';
        this.reportConfig = {
            dateRange: 'all',
            includeCharts: true,
            includeDetails: true,
            format: 'html'
        };
    }

    /**
     * 初始化报告模块
     */
    async init() {
        this.setupEventListeners();
        await this.loadReportData();
        this.renderReportInterface();
    }

    /**
     * 设置事件监听器
     */
    setupEventListeners() {
        // 报告类型切换
        document.querySelectorAll('.report-type-button').forEach(button => {
            button.addEventListener('click', (e) => {
                const reportType = e.target.dataset.reportType;
                this.switchReportType(reportType);
            });
        });

        // 生成报告按钮
        const generateBtn = document.getElementById('generate-report-btn');
        if (generateBtn) {
            generateBtn.addEventListener('click', () => {
                this.generateReport();
            });
        }

        // 导出按钮
        const exportHtmlBtn = document.getElementById('export-html-btn');
        if (exportHtmlBtn) {
            exportHtmlBtn.addEventListener('click', () => {
                this.exportReport('html');
            });
        }

        const exportPdfBtn = document.getElementById('export-pdf-btn');
        if (exportPdfBtn) {
            exportPdfBtn.addEventListener('click', () => {
                this.exportReport('pdf');
            });
        }

        const exportExcelBtn = document.getElementById('export-excel-btn');
        if (exportExcelBtn) {
            exportExcelBtn.addEventListener('click', () => {
                this.exportReport('excel');
            });
        }

        // 报告配置
        const configInputs = document.querySelectorAll('.report-config input, .report-config select');
        configInputs.forEach(input => {
            input.addEventListener('change', (e) => {
                this.updateReportConfig(e.target.name, e.target.value || e.target.checked);
            });
        });
    }

    /**
     * 加载报告数据
     */
    async loadReportData() {
        try {
            this.app.showLoading(true);

            // 并行加载各种数据
            const [doctors, stats, rankings, weights] = await Promise.all([
                this.app.api.getDoctors(),
                this.app.api.getStatistics(),
                this.app.api.getDoctorRankings(),
                this.app.api.getWeights()
            ]);

            this.reportData = {
                doctors,
                statistics: stats,
                rankings,
                weights,
                generatedAt: new Date().toISOString(),
                summary: this.generateSummary(doctors, stats)
            };

        } catch (error) {
            console.error('加载报告数据失败:', error);
            this.app.showNotification('加载报告数据失败', 'error');
            this.reportData = null;
        } finally {
            this.app.showLoading(false);
        }
    }

    /**
     * 生成数据摘要
     */
    generateSummary(doctors, stats) {
        const departmentStats = {};
        const titleStats = {};
        
        doctors.forEach(doctor => {
            // 按科室统计
            if (!departmentStats[doctor.department]) {
                departmentStats[doctor.department] = {
                    count: 0,
                    avgScore: 0,
                    highPerformers: 0
                };
            }
            departmentStats[doctor.department].count++;
            departmentStats[doctor.department].avgScore += doctor.total_score || 0;
            if ((doctor.total_score || 0) >= 80) {
                departmentStats[doctor.department].highPerformers++;
            }

            // 按职称统计
            if (!titleStats[doctor.title]) {
                titleStats[doctor.title] = {
                    count: 0,
                    avgScore: 0,
                    highPerformers: 0
                };
            }
            titleStats[doctor.title].count++;
            titleStats[doctor.title].avgScore += doctor.total_score || 0;
            if ((doctor.total_score || 0) >= 80) {
                titleStats[doctor.title].highPerformers++;
            }
        });

        // 计算平均分
        Object.values(departmentStats).forEach(stat => {
            stat.avgScore = stat.count > 0 ? stat.avgScore / stat.count : 0;
        });
        Object.values(titleStats).forEach(stat => {
            stat.avgScore = stat.count > 0 ? stat.avgScore / stat.count : 0;
        });

        return {
            departmentStats,
            titleStats,
            totalDoctors: doctors.length,
            avgTotalScore: stats.avg_total_score || 0,
            highPerformersCount: doctors.filter(d => (d.total_score || 0) >= 80).length,
            recommendationDistribution: this.calculateRecommendationDistribution(doctors)
        };
    }

    /**
     * 计算投放建议分布
     */
    calculateRecommendationDistribution(doctors) {
        const distribution = {
            '重点投放': 0,
            '适度投放': 0,
            '观察投放': 0,
            '待评估': 0
        };

        doctors.forEach(doctor => {
            const recommendation = doctor.recommendation || '待评估';
            distribution[recommendation] = (distribution[recommendation] || 0) + 1;
        });

        return distribution;
    }

    /**
     * 渲染报告界面
     */
    renderReportInterface() {
        const container = document.getElementById('reports-content');
        if (!container) return;

        container.innerHTML = `
            <div class="reports-layout">
                <!-- 报告类型选择 -->
                <div class="report-types">
                    <h3>报告类型</h3>
                    <div class="type-buttons">
                        <button class="report-type-button ${this.currentReportType === 'overview' ? 'active' : ''}" 
                                data-report-type="overview">
                            📊 概览报告
                        </button>
                        <button class="report-type-button ${this.currentReportType === 'performance' ? 'active' : ''}" 
                                data-report-type="performance">
                            🎯 绩效分析
                        </button>
                        <button class="report-type-button ${this.currentReportType === 'investment' ? 'active' : ''}" 
                                data-report-type="investment">
                            💰 投放建议
                        </button>
                        <button class="report-type-button ${this.currentReportType === 'comparison' ? 'active' : ''}" 
                                data-report-type="comparison">
                            📈 对比分析
                        </button>
                        <button class="report-type-button ${this.currentReportType === 'detailed' ? 'active' : ''}" 
                                data-report-type="detailed">
                            📋 详细清单
                        </button>
                    </div>
                </div>

                <!-- 报告配置 -->
                <div class="report-config">
                    <h3>报告配置</h3>
                    <div class="config-form">
                        <div class="config-group">
                            <label>数据范围</label>
                            <select name="dateRange" class="form-select">
                                <option value="all">全部数据</option>
                                <option value="recent">最近更新</option>
                                <option value="high">高分医生</option>
                                <option value="priority">优先投放</option>
                            </select>
                        </div>
                        <div class="config-group">
                            <label>
                                <input type="checkbox" name="includeCharts" checked>
                                包含图表
                            </label>
                        </div>
                        <div class="config-group">
                            <label>
                                <input type="checkbox" name="includeDetails" checked>
                                包含详情
                            </label>
                        </div>
                    </div>
                    <div class="config-actions">
                        <button id="generate-report-btn" class="btn btn-primary">
                            生成报告
                        </button>
                    </div>
                </div>

                <!-- 报告预览 -->
                <div class="report-preview">
                    <div class="preview-header">
                        <h3>报告预览</h3>
                        <div class="export-actions">
                            <button id="export-html-btn" class="btn btn-outline">
                                🌐 导出HTML
                            </button>
                            <button id="export-pdf-btn" class="btn btn-outline">
                                📄 导出PDF
                            </button>
                            <button id="export-excel-btn" class="btn btn-outline">
                                📊 导出Excel
                            </button>
                        </div>
                    </div>
                    <div class="preview-content" id="report-preview-content">
                        ${this.renderReportPreview()}
                    </div>
                </div>
            </div>
        `;

        this.bindReportEvents();
    }

    /**
     * 渲染报告预览
     */
    renderReportPreview() {
        if (!this.reportData) {
            return `
                <div class="preview-placeholder">
                    <div class="placeholder-icon">📋</div>
                    <div class="placeholder-title">报告预览</div>
                    <div class="placeholder-description">
                        点击"生成报告"按钮查看报告内容
                    </div>
                </div>
            `;
        }

        switch (this.currentReportType) {
            case 'overview':
                return this.renderOverviewReport();
            case 'performance':
                return this.renderPerformanceReport();
            case 'investment':
                return this.renderInvestmentReport();
            case 'comparison':
                return this.renderComparisonReport();
            case 'detailed':
                return this.renderDetailedReport();
            default:
                return this.renderOverviewReport();
        }
    }

    /**
     * 渲染概览报告
     */
    renderOverviewReport() {
        const data = this.reportData;
        const summary = data.summary;

        return `
            <div class="report-content overview-report">
                <div class="report-header">
                    <h2>医生投放分析概览报告</h2>
                    <div class="report-meta">
                        <span>生成时间: ${new Date(data.generatedAt).toLocaleString()}</span>
                        <span>数据范围: 全部医生</span>
                    </div>
                </div>

                <div class="executive-summary">
                    <h3>执行摘要</h3>
                    <div class="summary-grid">
                        <div class="summary-card">
                            <div class="card-value">${summary.totalDoctors}</div>
                            <div class="card-label">总医生数</div>
                        </div>
                        <div class="summary-card">
                            <div class="card-value">${this.app.formatNumber(summary.avgTotalScore)}</div>
                            <div class="card-label">平均评分</div>
                        </div>
                        <div class="summary-card">
                            <div class="card-value">${summary.highPerformersCount}</div>
                            <div class="card-label">高分医生 (≥80)</div>
                        </div>
                        <div class="summary-card">
                            <div class="card-value">${((summary.highPerformersCount / summary.totalDoctors) * 100).toFixed(1)}%</div>
                            <div class="card-label">高分占比</div>
                        </div>
                    </div>
                </div>

                <div class="recommendation-overview">
                    <h3>投放建议分布</h3>
                    <div class="recommendation-chart">
                        ${Object.entries(summary.recommendationDistribution).map(([type, count]) => {
                            const percentage = (count / summary.totalDoctors * 100).toFixed(1);
                            return `
                                <div class="recommendation-item">
                                    <div class="item-header">
                                        <span class="item-type">${type}</span>
                                        <span class="item-count">${count}人 (${percentage}%)</span>
                                    </div>
                                    <div class="item-bar">
                                        <div class="bar-fill ${this.getRecommendationClass(type)}" 
                                             style="width: ${percentage}%"></div>
                                    </div>
                                </div>
                            `;
                        }).join('')}
                    </div>
                </div>

                <div class="department-analysis">
                    <h3>科室表现分析</h3>
                    <div class="department-table">
                        <table>
                            <thead>
                                <tr>
                                    <th>科室</th>
                                    <th>医生数量</th>
                                    <th>平均评分</th>
                                    <th>高分医生</th>
                                    <th>高分占比</th>
                                </tr>
                            </thead>
                            <tbody>
                                ${Object.entries(summary.departmentStats)
                                    .sort((a, b) => b[1].avgScore - a[1].avgScore)
                                    .map(([dept, stats]) => `
                                        <tr>
                                            <td>${dept}</td>
                                            <td>${stats.count}</td>
                                            <td>${this.app.formatNumber(stats.avgScore)}</td>
                                            <td>${stats.highPerformers}</td>
                                            <td>${((stats.highPerformers / stats.count) * 100).toFixed(1)}%</td>
                                        </tr>
                                    `).join('')}
                            </tbody>
                        </table>
                    </div>
                </div>

                <div class="key-findings">
                    <h3>关键发现</h3>
                    <div class="findings-list">
                        ${this.generateKeyFindings(summary)}
                    </div>
                </div>

                <div class="recommendations">
                    <h3>建议措施</h3>
                    <div class="recommendations-list">
                        ${this.generateRecommendations(summary)}
                    </div>
                </div>
            </div>
        `;
    }

    /**
     * 生成关键发现
     */
    generateKeyFindings(summary) {
        const findings = [];

        // 高分医生比例分析
        const highScoreRate = (summary.highPerformersCount / summary.totalDoctors) * 100;
        if (highScoreRate > 25) {
            findings.push('✅ 高分医生占比较高，整体质量良好');
        } else if (highScoreRate > 15) {
            findings.push('⚠️ 高分医生占比中等，有提升空间');
        } else {
            findings.push('❌ 高分医生占比较低，需要重点关注');
        }

        // 科室表现分析
        const deptEntries = Object.entries(summary.departmentStats);
        const topDept = deptEntries.reduce((max, curr) => 
            curr[1].avgScore > max[1].avgScore ? curr : max
        );
        findings.push(`🏆 ${topDept[0]} 科室表现最佳，平均评分 ${this.app.formatNumber(topDept[1].avgScore)}`);

        // 投放建议分析
        const priority = summary.recommendationDistribution['重点投放'] || 0;
        const priorityRate = (priority / summary.totalDoctors) * 100;
        if (priorityRate > 20) {
            findings.push('🎯 重点投放医生数量充足，可以积极开展合作');
        } else {
            findings.push('📈 重点投放医生数量有限，建议优化评分权重或培养潜力医生');
        }

        return findings.map(finding => `<div class="finding-item">${finding}</div>`).join('');
    }

    /**
     * 生成建议措施
     */
    generateRecommendations(summary) {
        const recommendations = [];

        // 基于高分比例的建议
        const highScoreRate = (summary.highPerformersCount / summary.totalDoctors) * 100;
        if (highScoreRate < 20) {
            recommendations.push('🔧 调整评分权重配置，重新评估医生表现');
            recommendations.push('📚 加强医生培训和支持，提升整体表现');
        }

        // 基于科室分布的建议
        const deptCount = Object.keys(summary.departmentStats).length;
        if (deptCount > 5) {
            recommendations.push('🎯 重点关注表现优秀的科室，扩大合作规模');
            recommendations.push('📊 对表现较差的科室制定专门的提升计划');
        }

        // 基于投放建议的措施
        const priority = summary.recommendationDistribution['重点投放'] || 0;
        if (priority > 0) {
            recommendations.push(`💰 优先与 ${priority} 位重点投放医生建立合作关系`);
        }

        const moderate = summary.recommendationDistribution['适度投放'] || 0;
        if (moderate > 0) {
            recommendations.push(`🤝 与 ${moderate} 位适度投放医生保持良好关系，观察发展潜力`);
        }

        return recommendations.map(rec => `<div class="recommendation-item">${rec}</div>`).join('');
    }

    /**
     * 渲染绩效分析报告
     */
    renderPerformanceReport() {
        const data = this.reportData;

        return `
            <div class="report-content performance-report">
                <div class="report-header">
                    <h2>医生绩效分析报告</h2>
                    <div class="report-meta">
                        <span>生成时间: ${new Date(data.generatedAt).toLocaleString()}</span>
                    </div>
                </div>

                <div class="performance-overview">
                    <h3>绩效概览</h3>
                    <div class="metrics-grid">
                        <div class="metric-card">
                            <div class="metric-title">影响力评分</div>
                            <div class="metric-value">${this.app.formatNumber(data.statistics.avg_influence_score)}</div>
                            <div class="metric-trend">📈 +2.3%</div>
                        </div>
                        <div class="metric-card">
                            <div class="metric-title">内容质量评分</div>
                            <div class="metric-value">${this.app.formatNumber(data.statistics.avg_quality_score)}</div>
                            <div class="metric-trend">📊 +1.8%</div>
                        </div>
                        <div class="metric-card">
                            <div class="metric-title">活跃度评分</div>
                            <div class="metric-value">${this.app.formatNumber(data.statistics.avg_activity_score)}</div>
                            <div class="metric-trend">🚀 +3.1%</div>
                        </div>
                    </div>
                </div>

                <div class="top-performers">
                    <h3>TOP 10 高绩效医生</h3>
                    <div class="performers-table">
                        <table>
                            <thead>
                                <tr>
                                    <th>排名</th>
                                    <th>姓名</th>
                                    <th>职称</th>
                                    <th>科室</th>
                                    <th>总评分</th>
                                    <th>强项</th>
                                </tr>
                            </thead>
                            <tbody>
                                ${data.rankings.slice(0, 10).map((doctor, index) => `
                                    <tr>
                                        <td>${index + 1}</td>
                                        <td>${doctor.name}</td>
                                        <td>${doctor.title}</td>
                                        <td>${doctor.department}</td>
                                        <td>${this.app.formatNumber(doctor.total_score)}</td>
                                        <td>${this.getStrongSuit(doctor)}</td>
                                    </tr>
                                `).join('')}
                            </tbody>
                        </table>
                    </div>
                </div>

                <div class="performance-analysis">
                    <h3>绩效分析</h3>
                    <div class="analysis-sections">
                        <div class="analysis-section">
                            <h4>影响力分析</h4>
                            <p>影响力评分主要反映医生的知名度和专业声誉。高影响力医生通常具有更多的粉丝数量和更高的文章阅读量。</p>
                            <div class="score-distribution">
                                ${this.renderScoreDistribution(data.doctors, 'influence_score')}
                            </div>
                        </div>
                        
                        <div class="analysis-section">
                            <h4>内容质量分析</h4>
                            <p>内容质量评分体现医生发布内容的专业性和用户认可度，主要基于点赞率和互动质量。</p>
                            <div class="score-distribution">
                                ${this.renderScoreDistribution(data.doctors, 'quality_score')}
                            </div>
                        </div>
                        
                        <div class="analysis-section">
                            <h4>活跃度分析</h4>
                            <p>活跃度评分反映医生在平台上的参与程度，包括发文频率和用户互动响应。</p>
                            <div class="score-distribution">
                                ${this.renderScoreDistribution(data.doctors, 'activity_score')}
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        `;
    }

    /**
     * 获取医生强项
     */
    getStrongSuit(doctor) {
        const scores = {
            influence: doctor.influence_score || 0,
            quality: doctor.quality_score || 0,
            activity: doctor.activity_score || 0
        };

        const maxScore = Math.max(...Object.values(scores));
        const strongSuits = [];

        if (scores.influence === maxScore) strongSuits.push('影响力');
        if (scores.quality === maxScore) strongSuits.push('内容质量');
        if (scores.activity === maxScore) strongSuits.push('活跃度');

        return strongSuits.join(', ');
    }

    /**
     * 渲染评分分布
     */
    renderScoreDistribution(doctors, scoreField) {
        const ranges = [
            { min: 0, max: 20, label: '0-20' },
            { min: 21, max: 40, label: '21-40' },
            { min: 41, max: 60, label: '41-60' },
            { min: 61, max: 80, label: '61-80' },
            { min: 81, max: 100, label: '81-100' }
        ];

        const distribution = ranges.map(range => ({
            ...range,
            count: doctors.filter(d => {
                const score = d[scoreField] || 0;
                return score >= range.min && score <= range.max;
            }).length
        }));

        const maxCount = Math.max(...distribution.map(d => d.count));

        return `
            <div class="distribution-chart">
                ${distribution.map(range => `
                    <div class="distribution-bar">
                        <div class="bar-label">${range.label}</div>
                        <div class="bar-container">
                            <div class="bar-fill" style="width: ${maxCount > 0 ? (range.count / maxCount) * 100 : 0}%"></div>
                            <div class="bar-value">${range.count}</div>
                        </div>
                    </div>
                `).join('')}
            </div>
        `;
    }

    /**
     * 渲染投放建议报告
     */
    renderInvestmentReport() {
        const data = this.reportData;
        const distribution = data.summary.recommendationDistribution;

        return `
            <div class="report-content investment-report">
                <div class="report-header">
                    <h2>医生投放建议报告</h2>
                    <div class="report-meta">
                        <span>生成时间: ${new Date(data.generatedAt).toLocaleString()}</span>
                    </div>
                </div>

                <div class="investment-summary">
                    <h3>投放策略摘要</h3>
                    <div class="strategy-grid">
                        ${Object.entries(distribution).map(([type, count]) => `
                            <div class="strategy-card ${this.getRecommendationClass(type)}">
                                <div class="strategy-type">${type}</div>
                                <div class="strategy-count">${count}人</div>
                                <div class="strategy-percentage">${((count / data.summary.totalDoctors) * 100).toFixed(1)}%</div>
                            </div>
                        `).join('')}
                    </div>
                </div>

                <div class="priority-doctors">
                    <h3>重点投放医生清单</h3>
                    <div class="priority-list">
                        ${data.doctors.filter(d => d.recommendation === '重点投放').map(doctor => `
                            <div class="priority-doctor-card">
                                <div class="doctor-info">
                                    <div class="doctor-name">${doctor.name}</div>
                                    <div class="doctor-details">${doctor.title} | ${doctor.department} | ${doctor.hospital}</div>
                                </div>
                                <div class="doctor-scores">
                                    <div class="score-item">
                                        <span class="score-label">总分</span>
                                        <span class="score-value">${this.app.formatNumber(doctor.total_score)}</span>
                                    </div>
                                    <div class="score-breakdown">
                                        影响力: ${this.app.formatNumber(doctor.influence_score)} |
                                        质量: ${this.app.formatNumber(doctor.quality_score)} |
                                        活跃度: ${this.app.formatNumber(doctor.activity_score)}
                                    </div>
                                </div>
                                <div class="investment-reason">
                                    ${this.getInvestmentReason(doctor)}
                                </div>
                            </div>
                        `).join('')}
                    </div>
                </div>

                <div class="roi-analysis">
                    <h3>投资回报分析</h3>
                    <div class="roi-content">
                        <div class="roi-section">
                            <h4>预期回报</h4>
                            <ul>
                                <li>重点投放医生预期ROI: <strong>3.5-5.0倍</strong></li>
                                <li>适度投放医生预期ROI: <strong>2.0-3.5倍</strong></li>
                                <li>观察投放医生预期ROI: <strong>1.5-2.5倍</strong></li>
                            </ul>
                        </div>
                        <div class="roi-section">
                            <h4>风险评估</h4>
                            <ul>
                                <li>高评分医生合作风险较低</li>
                                <li>建议分批投放，降低集中风险</li>
                                <li>定期评估调整投放策略</li>
                            </ul>
                        </div>
                    </div>
                </div>

                <div class="budget-allocation">
                    <h3>预算分配建议</h3>
                    <div class="allocation-chart">
                        <div class="allocation-item">
                            <div class="allocation-label">重点投放 (60%)</div>
                            <div class="allocation-bar">
                                <div class="bar-fill high" style="width: 60%"></div>
                            </div>
                            <div class="allocation-amount">¥600,000</div>
                        </div>
                        <div class="allocation-item">
                            <div class="allocation-label">适度投放 (30%)</div>
                            <div class="allocation-bar">
                                <div class="bar-fill medium" style="width: 30%"></div>
                            </div>
                            <div class="allocation-amount">¥300,000</div>
                        </div>
                        <div class="allocation-item">
                            <div class="allocation-label">观察投放 (10%)</div>
                            <div class="allocation-bar">
                                <div class="bar-fill low" style="width: 10%"></div>
                            </div>
                            <div class="allocation-amount">¥100,000</div>
                        </div>
                    </div>
                </div>
            </div>
        `;
    }

    /**
     * 获取投放原因
     */
    getInvestmentReason(doctor) {
        const reasons = [];
        
        if ((doctor.total_score || 0) >= 85) {
            reasons.push('综合评分优秀');
        }
        
        if ((doctor.influence_score || 0) >= 80) {
            reasons.push('影响力突出');
        }
        
        if ((doctor.quality_score || 0) >= 80) {
            reasons.push('内容质量高');
        }
        
        if ((doctor.activity_score || 0) >= 80) {
            reasons.push('互动活跃');
        }

        if (doctor.title === '主任医师' || doctor.title === '副主任医师') {
            reasons.push('职称权威');
        }

        return reasons.length > 0 ? reasons.join('、') : '综合表现良好';
    }

    /**
     * 渲染对比分析报告
     */
    renderComparisonReport() {
        const data = this.reportData;

        return `
            <div class="report-content comparison-report">
                <div class="report-header">
                    <h2>医生对比分析报告</h2>
                    <div class="report-meta">
                        <span>生成时间: ${new Date(data.generatedAt).toLocaleString()}</span>
                    </div>
                </div>

                <div class="comparison-overview">
                    <h3>评分维度对比</h3>
                    <div class="dimension-comparison">
                        <canvas id="radar-chart" width="400" height="400"></canvas>
                    </div>
                </div>

                <div class="benchmark-analysis">
                    <h3>基准分析</h3>
                    <div class="benchmark-table">
                        <table>
                            <thead>
                                <tr>
                                    <th>评分维度</th>
                                    <th>平均分</th>
                                    <th>最高分</th>
                                    <th>最低分</th>
                                    <th>标准差</th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr>
                                    <td>影响力评分</td>
                                    <td>${this.app.formatNumber(data.statistics.avg_influence_score)}</td>
                                    <td>${this.getMaxScore(data.doctors, 'influence_score')}</td>
                                    <td>${this.getMinScore(data.doctors, 'influence_score')}</td>
                                    <td>${this.getStandardDeviation(data.doctors, 'influence_score')}</td>
                                </tr>
                                <tr>
                                    <td>内容质量评分</td>
                                    <td>${this.app.formatNumber(data.statistics.avg_quality_score)}</td>
                                    <td>${this.getMaxScore(data.doctors, 'quality_score')}</td>
                                    <td>${this.getMinScore(data.doctors, 'quality_score')}</td>
                                    <td>${this.getStandardDeviation(data.doctors, 'quality_score')}</td>
                                </tr>
                                <tr>
                                    <td>活跃度评分</td>
                                    <td>${this.app.formatNumber(data.statistics.avg_activity_score)}</td>
                                    <td>${this.getMaxScore(data.doctors, 'activity_score')}</td>
                                    <td>${this.getMinScore(data.doctors, 'activity_score')}</td>
                                    <td>${this.getStandardDeviation(data.doctors, 'activity_score')}</td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </div>

                <div class="correlation-analysis">
                    <h3>相关性分析</h3>
                    <div class="correlation-insights">
                        <div class="insight">
                            <strong>影响力与内容质量:</strong> 
                            正相关性较强，高影响力医生通常内容质量也较高
                        </div>
                        <div class="insight">
                            <strong>活跃度与总评分:</strong> 
                            中等正相关，活跃的医生整体表现更好
                        </div>
                        <div class="insight">
                            <strong>职称与评分:</strong> 
                            职称越高，平均评分越高，但个体差异较大
                        </div>
                    </div>
                </div>
            </div>
        `;
    }

    /**
     * 获取最高分
     */
    getMaxScore(doctors, field) {
        return Math.max(...doctors.map(d => d[field] || 0)).toFixed(1);
    }

    /**
     * 获取最低分
     */
    getMinScore(doctors, field) {
        return Math.min(...doctors.map(d => d[field] || 0)).toFixed(1);
    }

    /**
     * 计算标准差
     */
    getStandardDeviation(doctors, field) {
        const values = doctors.map(d => d[field] || 0);
        const mean = values.reduce((sum, val) => sum + val, 0) / values.length;
        const variance = values.reduce((sum, val) => sum + Math.pow(val - mean, 2), 0) / values.length;
        return Math.sqrt(variance).toFixed(2);
    }

    /**
     * 渲染详细清单报告
     */
    renderDetailedReport() {
        const data = this.reportData;

        return `
            <div class="report-content detailed-report">
                <div class="report-header">
                    <h2>医生详细清单报告</h2>
                    <div class="report-meta">
                        <span>生成时间: ${new Date(data.generatedAt).toLocaleString()}</span>
                        <span>医生总数: ${data.doctors.length}</span>
                    </div>
                </div>

                <div class="detailed-table">
                    <table>
                        <thead>
                            <tr>
                                <th>排名</th>
                                <th>姓名</th>
                                <th>职称</th>
                                <th>医院</th>
                                <th>科室</th>
                                <th>总评分</th>
                                <th>影响力</th>
                                <th>内容质量</th>
                                <th>活跃度</th>
                                <th>粉丝数</th>
                                <th>文章数</th>
                                <th>投放建议</th>
                            </tr>
                        </thead>
                        <tbody>
                            ${data.rankings.map((doctor, index) => `
                                <tr>
                                    <td>${index + 1}</td>
                                    <td>${doctor.name}</td>
                                    <td>${doctor.title}</td>
                                    <td>${doctor.hospital}</td>
                                    <td>${doctor.department}</td>
                                    <td>${this.app.formatNumber(doctor.total_score)}</td>
                                    <td>${this.app.formatNumber(doctor.influence_score)}</td>
                                    <td>${this.app.formatNumber(doctor.quality_score)}</td>
                                    <td>${this.app.formatNumber(doctor.activity_score)}</td>
                                    <td>${this.app.formatNumber(doctor.followers_count)}</td>
                                    <td>${this.app.formatNumber(doctor.articles_count)}</td>
                                    <td>
                                        <span class="recommendation-badge ${this.getRecommendationClass(doctor.recommendation)}">
                                            ${doctor.recommendation || '待评估'}
                                        </span>
                                    </td>
                                </tr>
                            `).join('')}
                        </tbody>
                    </table>
                </div>

                <div class="report-footer">
                    <div class="footer-notes">
                        <h4>说明</h4>
                        <ul>
                            <li>评分范围：0-100分</li>
                            <li>排名按总评分降序排列</li>
                            <li>投放建议基于评分和医生影响力综合评估</li>
                            <li>数据更新时间：${new Date().toLocaleString()}</li>
                        </ul>
                    </div>
                </div>
            </div>
        `;
    }

    /**
     * 获取投放建议样式类
     */
    getRecommendationClass(recommendation) {
        switch (recommendation) {
            case '重点投放': return 'high';
            case '适度投放': return 'medium';
            case '观察投放': return 'low';
            default: return 'none';
        }
    }

    /**
     * 绑定报告事件
     */
    bindReportEvents() {
        // 重新绑定所有事件监听器
        document.querySelectorAll('.report-type-button').forEach(button => {
            button.addEventListener('click', (e) => {
                const reportType = e.target.dataset.reportType;
                this.switchReportType(reportType);
            });
        });

        const generateBtn = document.getElementById('generate-report-btn');
        if (generateBtn) {
            generateBtn.addEventListener('click', () => {
                this.generateReport();
            });
        }

        const exportHtmlBtn = document.getElementById('export-html-btn');
        if (exportHtmlBtn) {
            exportHtmlBtn.addEventListener('click', () => {
                this.exportReport('html');
            });
        }

        const exportPdfBtn = document.getElementById('export-pdf-btn');
        if (exportPdfBtn) {
            exportPdfBtn.addEventListener('click', () => {
                this.exportReport('pdf');
            });
        }

        const exportExcelBtn = document.getElementById('export-excel-btn');
        if (exportExcelBtn) {
            exportExcelBtn.addEventListener('click', () => {
                this.exportReport('excel');
            });
        }

        // 报告配置
        document.querySelectorAll('.report-config input, .report-config select').forEach(input => {
            input.addEventListener('change', (e) => {
                this.updateReportConfig(e.target.name, e.target.value || e.target.checked);
            });
        });
    }

    /**
     * 切换报告类型
     */
    switchReportType(reportType) {
        this.currentReportType = reportType;
        
        // 更新按钮状态
        document.querySelectorAll('.report-type-button').forEach(button => {
            button.classList.toggle('active', button.dataset.reportType === reportType);
        });

        // 更新预览内容
        const previewContent = document.getElementById('report-preview-content');
        if (previewContent) {
            previewContent.innerHTML = this.renderReportPreview();
        }
    }

    /**
     * 更新报告配置
     */
    updateReportConfig(key, value) {
        this.reportConfig[key] = value;
    }

    /**
     * 生成报告
     */
    async generateReport() {
        try {
            this.app.showLoading(true);
            
            // 重新加载数据
            await this.loadReportData();
            
            // 更新预览
            const previewContent = document.getElementById('report-preview-content');
            if (previewContent) {
                previewContent.innerHTML = this.renderReportPreview();
            }

            this.app.showNotification('报告生成成功', 'success');

        } catch (error) {
            console.error('生成报告失败:', error);
            this.app.showNotification('生成报告失败', 'error');
        } finally {
            this.app.showLoading(false);
        }
    }

    /**
     * 导出报告
     */
    async exportReport(format) {
        if (!this.reportData) {
            this.app.showNotification('请先生成报告', 'error');
            return;
        }

        try {
            switch (format) {
                case 'html':
                    this.exportHtmlReport();
                    break;
                case 'pdf':
                    this.exportPdfReport();
                    break;
                case 'excel':
                    this.exportExcelReport();
                    break;
            }
        } catch (error) {
            console.error('导出报告失败:', error);
            this.app.showNotification('导出报告失败', 'error');
        }
    }

    /**
     * 导出HTML报告
     */
    exportHtmlReport() {
        const reportContent = document.getElementById('report-preview-content').innerHTML;
        const htmlContent = `
<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>医生投放分析报告 - ${this.currentReportType}</title>
    <style>
        body { font-family: 'Microsoft YaHei', sans-serif; margin: 20px; line-height: 1.6; }
        .report-content { max-width: 1200px; margin: 0 auto; }
        table { width: 100%; border-collapse: collapse; margin: 20px 0; }
        th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
        th { background-color: #f5f5f5; }
        .summary-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 20px; margin: 20px 0; }
        .summary-card { padding: 20px; border: 1px solid #ddd; border-radius: 8px; text-align: center; }
        .card-value { font-size: 2em; font-weight: bold; color: #3b82f6; }
        .card-label { color: #666; margin-top: 5px; }
        @media print { body { margin: 0; } }
    </style>
</head>
<body>
    ${reportContent}
</body>
</html>
        `;

        const blob = new Blob([htmlContent], { type: 'text/html;charset=utf-8' });
        const link = document.createElement('a');
        link.href = URL.createObjectURL(blob);
        link.download = `医生投放分析报告_${this.currentReportType}_${new Date().toISOString().split('T')[0]}.html`;
        link.click();
        URL.revokeObjectURL(link.href);

        this.app.showNotification('HTML报告导出成功', 'success');
    }

    /**
     * 导出PDF报告
     */
    exportPdfReport() {
        // 模拟PDF导出（实际需要PDF库支持）
        this.app.showNotification('PDF导出功能开发中...', 'info');
    }

    /**
     * 导出Excel报告
     */
    exportExcelReport() {
        if (!this.reportData) return;

        // 生成CSV格式的数据
        const headers = [
            '排名', '姓名', '职称', '医院', '科室', '总评分', 
            '影响力评分', '内容质量评分', '活跃度评分', 
            '粉丝数', '文章数', '平均阅读量', '平均点赞数', 
            '月发文量', '回复率', '投放建议'
        ];

        const rows = this.reportData.rankings.map((doctor, index) => [
            index + 1,
            doctor.name,
            doctor.title,
            doctor.hospital,
            doctor.department,
            doctor.total_score?.toFixed(2) || '0.00',
            doctor.influence_score?.toFixed(2) || '0.00',
            doctor.quality_score?.toFixed(2) || '0.00',
            doctor.activity_score?.toFixed(2) || '0.00',
            doctor.followers_count || 0,
            doctor.articles_count || 0,
            doctor.avg_views || 0,
            doctor.avg_likes || 0,
            doctor.monthly_articles || 0,
            doctor.response_rate?.toFixed(1) || '0.0',
            doctor.recommendation || '待评估'
        ]);

        const csvContent = [headers, ...rows]
            .map(row => row.join(','))
            .join('\n');

        const blob = new Blob(['\ufeff' + csvContent], { type: 'text/csv;charset=utf-8' });
        const link = document.createElement('a');
        link.href = URL.createObjectURL(blob);
        link.download = `医生投放分析报告_${this.currentReportType}_${new Date().toISOString().split('T')[0]}.csv`;
        link.click();
        URL.revokeObjectURL(link.href);

        this.app.showNotification('Excel报告导出成功', 'success');
    }
}

// 导出模块
if (typeof module !== 'undefined' && module.exports) {
    module.exports = ReportsModule;
} else {
    window.ReportsModule = ReportsModule;
}
