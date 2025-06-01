/**
 * 权重配置功能模块
 * 负责评分权重的配置、保存和预设方案管理
 */
class WeightsModule {
    constructor(app) {
        this.app = app;
        this.currentWeights = null;
        this.presetSchemes = [];
        this.hasUnsavedChanges = false;
    }

    /**
     * 初始化权重配置模块
     */
    async init() {
        await this.loadWeights();
        await this.loadPresetSchemes();
        this.setupEventListeners();
        this.renderWeightsConfig();
    }

    /**
     * 设置事件监听器
     */
    setupEventListeners() {
        // 权重滑块事件
        const weightSliders = ['influence-weight', 'quality-weight', 'activity-weight', 'title-weight'];
        weightSliders.forEach(sliderId => {
            const slider = document.getElementById(sliderId);
            if (slider) {
                slider.addEventListener('input', (e) => {
                    this.updateWeight(sliderId, parseFloat(e.target.value));
                });
            }
        });

        // 保存按钮
        const saveBtn = document.getElementById('save-weights-btn');
        if (saveBtn) {
            saveBtn.addEventListener('click', () => {
                this.saveWeights();
            });
        }

        // 重置按钮
        const resetBtn = document.getElementById('reset-weights-btn');
        if (resetBtn) {
            resetBtn.addEventListener('click', () => {
                this.resetWeights();
            });
        }

        // 预设方案选择
        const presetSelect = document.getElementById('preset-scheme-select');
        if (presetSelect) {
            presetSelect.addEventListener('change', (e) => {
                this.applyPresetScheme(e.target.value);
            });
        }

        // 保存为预设按钮
        const savePresetBtn = document.getElementById('save-preset-btn');
        if (savePresetBtn) {
            savePresetBtn.addEventListener('click', () => {
                this.showSavePresetModal();
            });
        }

        // 影响分析按钮
        const analyzeBtn = document.getElementById('analyze-impact-btn');
        if (analyzeBtn) {
            analyzeBtn.addEventListener('click', () => {
                this.analyzeImpact();
            });
        }

        // 监听页面离开事件
        window.addEventListener('beforeunload', (e) => {
            if (this.hasUnsavedChanges) {
                e.preventDefault();
                e.returnValue = '';
            }
        });
    }

    /**
     * 加载权重配置
     */
    async loadWeights() {
        try {
            this.currentWeights = await this.app.api.getWeights();
        } catch (error) {
            console.error('加载权重配置失败:', error);
            this.app.showNotification('加载权重配置失败', 'error');
            // 使用默认权重
            this.currentWeights = {
                influence_weight: 0.4,
                quality_weight: 0.3,
                activity_weight: 0.3,
                title_weight_factor: 0.1
            };
        }
    }

    /**
     * 加载预设方案
     */
    async loadPresetSchemes() {
        try {
            // 模拟预设方案数据
            this.presetSchemes = [
                {
                    id: 'balanced',
                    name: '均衡方案',
                    description: '各维度权重相对均衡',
                    influence_weight: 0.35,
                    quality_weight: 0.35,
                    activity_weight: 0.3,
                    title_weight_factor: 0.1
                },
                {
                    id: 'influence-focused',
                    name: '影响力优先',
                    description: '重点关注医生影响力',
                    influence_weight: 0.5,
                    quality_weight: 0.25,
                    activity_weight: 0.25,
                    title_weight_factor: 0.15
                },
                {
                    id: 'quality-focused',
                    name: '质量优先',
                    description: '重点关注内容质量',
                    influence_weight: 0.25,
                    quality_weight: 0.5,
                    activity_weight: 0.25,
                    title_weight_factor: 0.1
                },
                {
                    id: 'activity-focused',
                    name: '活跃度优先',
                    description: '重点关注医生活跃度',
                    influence_weight: 0.25,
                    quality_weight: 0.25,
                    activity_weight: 0.5,
                    title_weight_factor: 0.1
                }
            ];
        } catch (error) {
            console.error('加载预设方案失败:', error);
            this.presetSchemes = [];
        }
    }

    /**
     * 渲染权重配置界面
     */
    renderWeightsConfig() {
        const container = document.getElementById('weights-content');
        if (!container) return;

        container.innerHTML = `
            <div class="weights-config">
                <!-- 权重配置面板 -->
                <div class="config-panel">
                    <div class="panel-header">
                        <h3>权重配置</h3>
                        <div class="panel-actions">
                            <select id="preset-scheme-select" class="form-select">
                                <option value="">选择预设方案</option>
                                ${this.presetSchemes.map(scheme => `
                                    <option value="${scheme.id}">${scheme.name}</option>
                                `).join('')}
                            </select>
                            <button id="save-preset-btn" class="btn btn-outline">保存为预设</button>
                        </div>
                    </div>
                    
                    <div class="weights-form">
                        ${this.renderWeightSlider('influence-weight', '影响力权重', this.currentWeights.influence_weight, '基于粉丝数、文章阅读量等指标')}
                        ${this.renderWeightSlider('quality-weight', '内容质量权重', this.currentWeights.quality_weight, '基于文章点赞率、评论互动等指标')}
                        ${this.renderWeightSlider('activity-weight', '活跃度权重', this.currentWeights.activity_weight, '基于发文频率、回复率等指标')}
                        ${this.renderWeightSlider('title-weight', '职称加权系数', this.currentWeights.title_weight_factor, '职称对总评分的加权影响')}
                    </div>
                    
                    <div class="weight-summary">
                        <div class="summary-item">
                            <span class="summary-label">权重总和:</span>
                            <span class="summary-value" id="weights-sum">
                                ${((this.currentWeights.influence_weight + this.currentWeights.quality_weight + this.currentWeights.activity_weight) * 100).toFixed(1)}%
                            </span>
                        </div>
                        <div class="summary-warning" id="weight-warning" style="display: none;">
                            ⚠️ 主要权重总和应该等于100%
                        </div>
                    </div>
                    
                    <div class="panel-actions">
                        <button id="reset-weights-btn" class="btn btn-outline">重置</button>
                        <button id="analyze-impact-btn" class="btn btn-outline">影响分析</button>
                        <button id="save-weights-btn" class="btn btn-primary" ${!this.hasUnsavedChanges ? 'disabled' : ''}>
                            保存配置
                        </button>
                    </div>
                </div>
                
                <!-- 预览面板 -->
                <div class="preview-panel">
                    <div class="panel-header">
                        <h3>配置预览</h3>
                    </div>
                    
                    <div class="preview-content">
                        <div class="preview-chart">
                            <h4>权重分布</h4>
                            ${this.renderWeightChart()}
                        </div>
                        
                        <div class="preview-impact">
                            <h4>配置说明</h4>
                            ${this.renderConfigDescription()}
                        </div>
                        
                        <div class="preview-examples">
                            <h4>评分示例</h4>
                            ${this.renderScoreExamples()}
                        </div>
                    </div>
                </div>
            </div>
        `;

        this.updateWeightsSummary();
    }

    /**
     * 渲染权重滑块
     */
    renderWeightSlider(id, label, value, description) {
        const percentage = (value * 100).toFixed(1);
        
        return `
            <div class="weight-group">
                <div class="weight-header">
                    <label for="${id}" class="weight-label">${label}</label>
                    <span class="weight-value" id="${id}-value">${percentage}%</span>
                </div>
                <div class="weight-slider-container">
                    <input type="range" 
                           id="${id}" 
                           class="weight-slider" 
                           min="0" 
                           max="${id === 'title-weight' ? '0.3' : '1'}" 
                           step="0.01" 
                           value="${value}">
                </div>
                <div class="weight-description">${description}</div>
            </div>
        `;
    }

    /**
     * 渲染权重图表
     */
    renderWeightChart() {
        const weights = [
            { name: '影响力', value: this.currentWeights.influence_weight, color: '#3b82f6' },
            { name: '内容质量', value: this.currentWeights.quality_weight, color: '#10b981' },
            { name: '活跃度', value: this.currentWeights.activity_weight, color: '#f59e0b' }
        ];

        const total = weights.reduce((sum, w) => sum + w.value, 0);
        
        return `
            <div class="pie-chart">
                ${weights.map(weight => {
                    const percentage = total > 0 ? (weight.value / total * 100) : 0;
                    return `
                        <div class="pie-segment" style="background-color: ${weight.color}; flex: ${weight.value}">
                            <span class="segment-label">${weight.name}</span>
                            <span class="segment-value">${percentage.toFixed(1)}%</span>
                        </div>
                    `;
                }).join('')}
            </div>
        `;
    }

    /**
     * 渲染配置说明
     */
    renderConfigDescription() {
        let description = '';
        
        const maxWeight = Math.max(
            this.currentWeights.influence_weight,
            this.currentWeights.quality_weight,
            this.currentWeights.activity_weight
        );

        if (this.currentWeights.influence_weight === maxWeight) {
            description = '当前配置偏向<strong>影响力优先</strong>，适合寻找具有较大影响力的医生';
        } else if (this.currentWeights.quality_weight === maxWeight) {
            description = '当前配置偏向<strong>质量优先</strong>，适合寻找内容质量高的医生';
        } else if (this.currentWeights.activity_weight === maxWeight) {
            description = '当前配置偏向<strong>活跃度优先</strong>，适合寻找互动活跃的医生';
        } else {
            description = '当前配置<strong>相对均衡</strong>，综合考虑各项指标';
        }

        return `
            <div class="config-description">
                <p>${description}</p>
                <div class="config-details">
                    <div class="detail-item">
                        <span class="detail-label">职称加权:</span>
                        <span class="detail-value">${(this.currentWeights.title_weight_factor * 100).toFixed(1)}%</span>
                    </div>
                    <div class="detail-description">
                        职称越高的医生，其评分将获得额外加成
                    </div>
                </div>
            </div>
        `;
    }

    /**
     * 渲染评分示例
     */
    renderScoreExamples() {
        const examples = [
            {
                name: '张医生',
                title: '主任医师',
                influence: 85,
                quality: 75,
                activity: 80
            },
            {
                name: '李医生',
                title: '主治医师',
                influence: 70,
                quality: 90,
                activity: 65
            },
            {
                name: '王医生',
                title: '住院医师',
                influence: 60,
                quality: 70,
                activity: 95
            }
        ];

        return `
            <div class="score-examples">
                ${examples.map(example => {
                    const score = this.calculateExampleScore(example);
                    return `
                        <div class="example-item">
                            <div class="example-header">
                                <span class="example-name">${example.name}</span>
                                <span class="example-title">${example.title}</span>
                                <span class="example-score">${score.toFixed(1)}</span>
                            </div>
                            <div class="example-breakdown">
                                <small>
                                    影响力: ${example.influence} | 
                                    质量: ${example.quality} | 
                                    活跃度: ${example.activity}
                                </small>
                            </div>
                        </div>
                    `;
                }).join('')}
            </div>
        `;
    }

    /**
     * 计算示例评分
     */
    calculateExampleScore(example) {
        const baseScore = 
            example.influence * this.currentWeights.influence_weight +
            example.quality * this.currentWeights.quality_weight +
            example.activity * this.currentWeights.activity_weight;

        // 职称加权
        const titleMultiplier = this.getTitleMultiplier(example.title);
        
        return baseScore * (1 + titleMultiplier * this.currentWeights.title_weight_factor);
    }

    /**
     * 获取职称倍数
     */
    getTitleMultiplier(title) {
        const titleMap = {
            '主任医师': 1.5,
            '副主任医师': 1.2,
            '主治医师': 1.0,
            '住院医师': 0.8
        };
        return titleMap[title] || 1.0;
    }

    /**
     * 更新权重
     */
    updateWeight(weightId, value) {
        const weightMap = {
            'influence-weight': 'influence_weight',
            'quality-weight': 'quality_weight',
            'activity-weight': 'activity_weight',
            'title-weight': 'title_weight_factor'
        };

        const weightKey = weightMap[weightId];
        if (weightKey) {
            this.currentWeights[weightKey] = value;
            this.hasUnsavedChanges = true;
            
            // 更新显示值
            const valueElement = document.getElementById(`${weightId}-value`);
            if (valueElement) {
                valueElement.textContent = `${(value * 100).toFixed(1)}%`;
            }
            
            // 更新保存按钮状态
            const saveBtn = document.getElementById('save-weights-btn');
            if (saveBtn) {
                saveBtn.disabled = false;
            }
            
            // 更新预览
            this.updateWeightsSummary();
            this.updatePreview();
        }
    }

    /**
     * 更新权重汇总
     */
    updateWeightsSummary() {
        const sum = this.currentWeights.influence_weight + 
                   this.currentWeights.quality_weight + 
                   this.currentWeights.activity_weight;
        
        const sumElement = document.getElementById('weights-sum');
        if (sumElement) {
            sumElement.textContent = `${(sum * 100).toFixed(1)}%`;
            
            // 根据总和显示警告
            const warningElement = document.getElementById('weight-warning');
            if (warningElement) {
                const isValid = Math.abs(sum - 1) < 0.01;
                warningElement.style.display = isValid ? 'none' : 'block';
                sumElement.style.color = isValid ? 'var(--text-color)' : 'var(--danger-color)';
            }
        }
    }

    /**
     * 更新预览
     */
    updatePreview() {
        // 更新权重图表
        const chartContainer = document.querySelector('.pie-chart');
        if (chartContainer) {
            chartContainer.innerHTML = this.renderWeightChart().match(/<div class="pie-chart">(.*?)<\/div>/s)[1];
        }

        // 更新配置说明
        const descContainer = document.querySelector('.config-description');
        if (descContainer) {
            descContainer.innerHTML = this.renderConfigDescription().match(/<div class="config-description">(.*?)<\/div>/s)[1];
        }

        // 更新评分示例
        const examplesContainer = document.querySelector('.score-examples');
        if (examplesContainer) {
            examplesContainer.innerHTML = this.renderScoreExamples().match(/<div class="score-examples">(.*?)<\/div>/s)[1];
        }
    }

    /**
     * 应用预设方案
     */
    applyPresetScheme(schemeId) {
        if (!schemeId) return;

        const scheme = this.presetSchemes.find(s => s.id === schemeId);
        if (!scheme) return;

        // 更新权重值
        this.currentWeights.influence_weight = scheme.influence_weight;
        this.currentWeights.quality_weight = scheme.quality_weight;
        this.currentWeights.activity_weight = scheme.activity_weight;
        this.currentWeights.title_weight_factor = scheme.title_weight_factor;

        // 更新滑块
        document.getElementById('influence-weight').value = scheme.influence_weight;
        document.getElementById('quality-weight').value = scheme.quality_weight;
        document.getElementById('activity-weight').value = scheme.activity_weight;
        document.getElementById('title-weight').value = scheme.title_weight_factor;

        // 更新显示
        document.getElementById('influence-weight-value').textContent = `${(scheme.influence_weight * 100).toFixed(1)}%`;
        document.getElementById('quality-weight-value').textContent = `${(scheme.quality_weight * 100).toFixed(1)}%`;
        document.getElementById('activity-weight-value').textContent = `${(scheme.activity_weight * 100).toFixed(1)}%`;
        document.getElementById('title-weight-value').textContent = `${(scheme.title_weight_factor * 100).toFixed(1)}%`;

        this.hasUnsavedChanges = true;
        
        // 更新保存按钮状态
        const saveBtn = document.getElementById('save-weights-btn');
        if (saveBtn) {
            saveBtn.disabled = false;
        }

        this.updateWeightsSummary();
        this.updatePreview();

        this.app.showNotification(`已应用"${scheme.name}"方案`, 'success');
    }

    /**
     * 保存权重配置
     */
    async saveWeights() {
        try {
            // 验证权重总和
            const sum = this.currentWeights.influence_weight + 
                       this.currentWeights.quality_weight + 
                       this.currentWeights.activity_weight;
            
            if (Math.abs(sum - 1) > 0.01) {
                this.app.showNotification('主要权重总和必须等于100%', 'error');
                return;
            }

            await this.app.api.updateWeights(this.currentWeights);
            this.hasUnsavedChanges = false;
            
            // 更新保存按钮状态
            const saveBtn = document.getElementById('save-weights-btn');
            if (saveBtn) {
                saveBtn.disabled = true;
            }

            this.app.showNotification('权重配置保存成功', 'success');
            
            // 询问是否重新计算评分
            if (confirm('权重配置已保存，是否要重新计算所有医生的评分？')) {
                try {
                    await this.app.api.recalculateScores();
                    this.app.showNotification('评分重新计算完成', 'success');
                } catch (error) {
                    console.error('重新计算评分失败:', error);
                    this.app.showNotification('重新计算评分失败', 'error');
                }
            }

        } catch (error) {
            console.error('保存权重配置失败:', error);
            this.app.showNotification('保存权重配置失败', 'error');
        }
    }

    /**
     * 重置权重配置
     */
    resetWeights() {
        if (!confirm('确定要重置权重配置吗？未保存的更改将丢失。')) {
            return;
        }

        // 重置为默认值
        this.currentWeights = {
            influence_weight: 0.4,
            quality_weight: 0.3,
            activity_weight: 0.3,
            title_weight_factor: 0.1
        };

        this.hasUnsavedChanges = false;
        this.renderWeightsConfig();
        this.app.showNotification('权重配置已重置', 'success');
    }

    /**
     * 显示保存预设模态框
     */
    showSavePresetModal() {
        const modalHtml = `
            <div class="modal-overlay" id="save-preset-modal">
                <div class="modal">
                    <div class="modal-header">
                        <h3>保存为预设方案</h3>
                        <button class="modal-close" onclick="weightsModule.closeSavePresetModal()">×</button>
                    </div>
                    <div class="modal-body">
                        <form id="save-preset-form" class="form">
                            <div class="form-group">
                                <label for="preset-name">方案名称 *</label>
                                <input type="text" id="preset-name" class="form-input" required 
                                       placeholder="输入方案名称">
                            </div>
                            <div class="form-group">
                                <label for="preset-description">方案描述</label>
                                <textarea id="preset-description" class="form-textarea" 
                                          placeholder="描述该方案的特点和适用场景"></textarea>
                            </div>
                            <div class="form-group">
                                <label>当前权重配置</label>
                                <div class="weight-preview">
                                    <div class="preview-item">
                                        <span>影响力权重:</span>
                                        <span>${(this.currentWeights.influence_weight * 100).toFixed(1)}%</span>
                                    </div>
                                    <div class="preview-item">
                                        <span>内容质量权重:</span>
                                        <span>${(this.currentWeights.quality_weight * 100).toFixed(1)}%</span>
                                    </div>
                                    <div class="preview-item">
                                        <span>活跃度权重:</span>
                                        <span>${(this.currentWeights.activity_weight * 100).toFixed(1)}%</span>
                                    </div>
                                    <div class="preview-item">
                                        <span>职称加权系数:</span>
                                        <span>${(this.currentWeights.title_weight_factor * 100).toFixed(1)}%</span>
                                    </div>
                                </div>
                            </div>
                        </form>
                    </div>
                    <div class="modal-footer">
                        <button type="button" class="btn btn-outline" onclick="weightsModule.closeSavePresetModal()">
                            取消
                        </button>
                        <button type="button" class="btn btn-primary" onclick="weightsModule.savePresetScheme()">
                            保存
                        </button>
                    </div>
                </div>
            </div>
        `;

        document.body.insertAdjacentHTML('beforeend', modalHtml);
    }

    /**
     * 保存预设方案
     */
    savePresetScheme() {
        const name = document.getElementById('preset-name').value.trim();
        const description = document.getElementById('preset-description').value.trim();

        if (!name) {
            this.app.showNotification('请输入方案名称', 'error');
            return;
        }

        // 创建新的预设方案
        const newScheme = {
            id: `custom_${Date.now()}`,
            name: name,
            description: description || `自定义方案: ${name}`,
            influence_weight: this.currentWeights.influence_weight,
            quality_weight: this.currentWeights.quality_weight,
            activity_weight: this.currentWeights.activity_weight,
            title_weight_factor: this.currentWeights.title_weight_factor
        };

        // 添加到预设方案列表
        this.presetSchemes.push(newScheme);

        // 更新预设选择器
        const presetSelect = document.getElementById('preset-scheme-select');
        if (presetSelect) {
            const option = document.createElement('option');
            option.value = newScheme.id;
            option.textContent = newScheme.name;
            presetSelect.appendChild(option);
        }

        this.closeSavePresetModal();
        this.app.showNotification('预设方案保存成功', 'success');
    }

    /**
     * 关闭保存预设模态框
     */
    closeSavePresetModal() {
        const modal = document.getElementById('save-preset-modal');
        if (modal) {
            modal.remove();
        }
    }

    /**
     * 分析权重影响
     */
    async analyzeImpact() {
        try {
            this.app.showLoading(true);
            
            // 模拟影响分析
            await new Promise(resolve => setTimeout(resolve, 1000));
            
            const analysisHtml = `
                <div class="modal-overlay" id="impact-analysis-modal">
                    <div class="modal modal-large">
                        <div class="modal-header">
                            <h3>权重影响分析</h3>
                            <button class="modal-close" onclick="weightsModule.closeImpactAnalysis()">×</button>
                        </div>
                        <div class="modal-body">
                            <div class="impact-analysis">
                                <div class="analysis-section">
                                    <h4>排名变化预测</h4>
                                    <div class="ranking-changes">
                                        <div class="change-item positive">
                                            <span class="doctor-name">张主任</span>
                                            <span class="change-indicator">↑ 3位</span>
                                            <span class="change-reason">影响力权重增加</span>
                                        </div>
                                        <div class="change-item negative">
                                            <span class="doctor-name">李医生</span>
                                            <span class="change-indicator">↓ 2位</span>
                                            <span class="change-reason">活跃度权重降低</span>
                                        </div>
                                        <div class="change-item positive">
                                            <span class="doctor-name">王医生</span>
                                            <span class="change-indicator">↑ 1位</span>
                                            <span class="change-reason">质量权重提升</span>
                                        </div>
                                    </div>
                                </div>
                                
                                <div class="analysis-section">
                                    <h4>投放建议变化</h4>
                                    <div class="recommendation-changes">
                                        <div class="change-summary">
                                            <div class="summary-item">
                                                <span class="count">+3</span>
                                                <span class="label">新增重点投放</span>
                                            </div>
                                            <div class="summary-item">
                                                <span class="count">-1</span>
                                                <span class="label">降级适度投放</span>
                                            </div>
                                            <div class="summary-item">
                                                <span class="count">+2</span>
                                                <span class="label">升级观察投放</span>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                                
                                <div class="analysis-section">
                                    <h4>评分分布变化</h4>
                                    <div class="distribution-comparison">
                                        <div class="distribution-chart">
                                            <div class="chart-title">当前分布</div>
                                            <div class="score-bars">
                                                <div class="score-bar" style="height: 30%">
                                                    <span class="bar-label">80+</span>
                                                    <span class="bar-value">18%</span>
                                                </div>
                                                <div class="score-bar" style="height: 60%">
                                                    <span class="bar-label">60-80</span>
                                                    <span class="bar-value">55%</span>
                                                </div>
                                                <div class="score-bar" style="height: 40%">
                                                    <span class="bar-label">60-</span>
                                                    <span class="bar-value">27%</span>
                                                </div>
                                            </div>
                                        </div>
                                        <div class="distribution-chart">
                                            <div class="chart-title">预测分布</div>
                                            <div class="score-bars">
                                                <div class="score-bar" style="height: 35%">
                                                    <span class="bar-label">80+</span>
                                                    <span class="bar-value">22%</span>
                                                </div>
                                                <div class="score-bar" style="height: 65%">
                                                    <span class="bar-label">60-80</span>
                                                    <span class="bar-value">58%</span>
                                                </div>
                                                <div class="score-bar" style="height: 30%">
                                                    <span class="bar-label">60-</span>
                                                    <span class="bar-value">20%</span>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="modal-footer">
                            <button type="button" class="btn btn-outline" onclick="weightsModule.closeImpactAnalysis()">
                                关闭
                            </button>
                            <button type="button" class="btn btn-primary" onclick="weightsModule.applyWeightsAndCalculate()">
                                应用权重并重新计算
                            </button>
                        </div>
                    </div>
                </div>
            `;

            document.body.insertAdjacentHTML('beforeend', analysisHtml);

        } catch (error) {
            console.error('分析权重影响失败:', error);
            this.app.showNotification('分析失败', 'error');
        } finally {
            this.app.showLoading(false);
        }
    }

    /**
     * 关闭影响分析
     */
    closeImpactAnalysis() {
        const modal = document.getElementById('impact-analysis-modal');
        if (modal) {
            modal.remove();
        }
    }

    /**
     * 应用权重并重新计算
     */
    async applyWeightsAndCalculate() {
        this.closeImpactAnalysis();
        await this.saveWeights();
    }
}

// 导出模块
if (typeof module !== 'undefined' && module.exports) {
    module.exports = WeightsModule;
} else {
    window.WeightsModule = WeightsModule;
}
