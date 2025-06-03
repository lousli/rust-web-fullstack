/**
 * 医疗投放评价系统 - 权重配置管理模块
 * 支持五大核心评价指标的权重配置
 */
class MedicalWeightsModule {
    constructor(app) {
        this.app = app || window.app; // 兼容独立页面和主应用
        this.currentWeights = null;
        this.presetSchemes = [];
        this.hasUnsavedChanges = false;
        this.isAnalyzing = false;
        
        // 五大核心评价指标
        this.coreIndicators = {
            account_influence: {
                name: '账号影响力',
                description: '基于粉丝数量、互动数据和影响力指数',
                defaultWeight: 22.0,
                minWeight: 5.0,
                maxWeight: 50.0
            },
            cost_effectiveness: {
                name: '性价比指数', 
                description: '综合考虑报价与影响力的性价比',
                defaultWeight: 35.0,
                minWeight: 10.0,
                maxWeight: 60.0
            },
            content_quality: {
                name: '内容质量',
                description: '包含表现力、剪辑水平、画面质量等',
                defaultWeight: 28.0,
                minWeight: 15.0,
                maxWeight: 50.0
            },
            medical_credibility: {
                name: '医疗可信度',
                description: '基于职称、科室和专业性评分',
                defaultWeight: 10.0,
                minWeight: 5.0,
                maxWeight: 30.0
            },
            roi_prediction: {
                name: 'ROI预测',
                description: '基于历史数据预测投放回报率',
                defaultWeight: 5.0,
                minWeight: 2.0,
                maxWeight: 25.0
            }
        };
    }

    /**
     * 初始化医疗权重配置模块
     */
    async init() {
        await this.loadCurrentWeights();
        await this.loadPresetSchemes();
        this.setupEventListeners();
        this.renderWeightsInterface();
        this.updateWeightSummary();
    }

    /**
     * 加载当前权重配置
     */
    async loadCurrentWeights() {
        try {
            const response = await fetch('/api/weight-configs/active');
            if (response.ok) {
                const result = await response.json();
                if (result.success) {
                    // 转换为医疗权重格式
                    this.currentWeights = this.convertToMedicalWeights(result.data);
                } else {
                    throw new Error(result.message);
                }
            } else {
                throw new Error('获取权重配置失败');
            }
        } catch (error) {
            console.error('加载权重配置失败:', error);
            this.app.showNotification('加载权重配置失败: ' + error.message, 'error');
            // 使用默认权重
            this.currentWeights = this.getDefaultWeights();
        }
    }

    /**
     * 加载预设方案
     */
    async loadPresetSchemes() {
        try {
            const response = await fetch('/api/weight-configs/medical/presets');
            if (response.ok) {
                const result = await response.json();
                if (result.success) {
                    this.presetSchemes = result.data;
                } else {
                    throw new Error(result.message);
                }
            } else {
                throw new Error('获取预设方案失败');
            }
        } catch (error) {
            console.error('加载预设方案失败:', error);
            this.presetSchemes = this.getDefaultPresets();
        }
    }

    /**
     * 获取默认权重配置
     */
    getDefaultWeights() {
        const weights = {};
        Object.keys(this.coreIndicators).forEach(key => {
            weights[key + '_weight'] = this.coreIndicators[key].defaultWeight;
        });
        return weights;
    }

    /**
     * 获取默认预设方案
     */
    getDefaultPresets() {
        return [
            {
                id: 'balanced',
                name: '平衡型投放',
                description: '标准医疗健康领域权重配置，各指标权重均衡',
                strategy: 'Balanced',
                weights: {
                    account_influence_weight: 22.0,
                    cost_effectiveness_weight: 35.0,
                    content_quality_weight: 28.0,
                    medical_credibility_weight: 10.0,
                    roi_prediction_weight: 5.0
                },
                suitable_scenarios: ['通用场景', '综合考量', '长期合作']
            }
        ];
    }

    /**
     * 转换通用权重配置为医疗权重格式
     */
    convertToMedicalWeights(config) {
        return {
            account_influence_weight: config.account_type_weight || 22.0,
            cost_effectiveness_weight: config.cost_effectiveness_weight || 35.0,
            content_quality_weight: (config.performance_weight + config.affinity_weight + 
                                   config.editing_weight + config.video_quality_weight) || 28.0,
            medical_credibility_weight: 10.0, // 新指标，从其他指标中分配
            roi_prediction_weight: 5.0 // 新指标，从其他指标中分配
        };
    }

    /**
     * 设置事件监听器
     */
    setupEventListeners() {
        // 权重滑块事件
        Object.keys(this.coreIndicators).forEach(indicator => {
            const slider = document.getElementById(`${indicator}-weight-slider`);
            const input = document.getElementById(`${indicator}-weight-input`);
            
            if (slider) {
                slider.addEventListener('input', (e) => {
                    this.updateWeight(indicator, parseFloat(e.target.value));
                });
            }
            
            if (input) {
                input.addEventListener('change', (e) => {
                    this.updateWeight(indicator, parseFloat(e.target.value));
                });
            }
        });

        // 预设方案选择
        const presetSelect = document.getElementById('preset-scheme-select');
        if (presetSelect) {
            presetSelect.addEventListener('change', (e) => {
                this.applyPresetScheme(e.target.value);
            });
        }

        // 保存权重配置
        const saveBtn = document.getElementById('save-weights-btn');
        if (saveBtn) {
            saveBtn.addEventListener('click', () => {
                this.showSaveWeightsModal();
            });
        }

        // 重置权重
        const resetBtn = document.getElementById('reset-weights-btn');
        if (resetBtn) {
            resetBtn.addEventListener('click', () => {
                this.resetWeights();
            });
        }

        // 权重影响分析
        const analyzeBtn = document.getElementById('analyze-impact-btn');
        if (analyzeBtn) {
            analyzeBtn.addEventListener('click', () => {
                this.analyzeWeightImpact();
            });
        }

        // 实时预览
        const previewBtn = document.getElementById('preview-weights-btn');
        if (previewBtn) {
            previewBtn.addEventListener('click', () => {
                this.previewWeightImpact();
            });
        }

        // 页面离开时检查未保存更改
        window.addEventListener('beforeunload', (e) => {
            if (this.hasUnsavedChanges) {
                e.preventDefault();
                e.returnValue = '您有未保存的权重配置更改，确定要离开吗？';
            }
        });
    }

    /**
     * 渲染权重配置界面
     */
    renderWeightsInterface() {
        const container = document.getElementById('weights-content');
        if (!container) return;

        container.innerHTML = `
            <div class="medical-weights-config">
                <!-- 头部说明 -->
                <div class="config-header">
                    <h2>医疗投放评价权重配置</h2>
                    <p class="config-description">
                        基于五大核心评价指标配置医生投放评价权重，权重总和必须为100%
                    </p>
                </div>

                <!-- 预设方案选择 -->
                <div class="preset-section">
                    <div class="section-header">
                        <h3>预设方案</h3>
                        <select id="preset-scheme-select" class="form-select">
                            <option value="">选择预设投放策略</option>
                            ${this.presetSchemes.map(scheme => `
                                <option value="${scheme.id}" data-strategy="${scheme.strategy}">
                                    ${scheme.name} - ${scheme.description}
                                </option>
                            `).join('')}
                        </select>
                    </div>
                </div>

                <!-- 权重配置面板 -->
                <div class="weights-panel">
                    <div class="weights-grid">
                        ${Object.keys(this.coreIndicators).map(indicator => 
                            this.renderWeightControl(indicator)
                        ).join('')}
                    </div>
                    
                    <!-- 权重汇总 -->
                    <div class="weight-summary">
                        <div class="summary-header">
                            <h4>权重分布汇总</h4>
                            <div class="total-weight">
                                <span class="label">总计:</span>
                                <span id="total-weight-value" class="value">100.0%</span>
                            </div>
                        </div>
                        <div class="summary-chart" id="weight-chart"></div>
                        <div id="weight-validation" class="validation-message"></div>
                    </div>
                </div>

                <!-- 影响分析面板 -->
                <div class="analysis-panel" id="analysis-panel" style="display: none;">
                    <div class="panel-header">
                        <h4>权重影响分析</h4>
                        <button id="close-analysis-btn" class="btn-close">&times;</button>
                    </div>
                    <div id="analysis-content"></div>
                </div>

                <!-- 操作按钮 -->
                <div class="config-actions">
                    <button id="reset-weights-btn" class="btn btn-secondary">
                        <i class="icon-reset"></i> 重置默认
                    </button>
                    <button id="preview-weights-btn" class="btn btn-outline">
                        <i class="icon-preview"></i> 实时预览
                    </button>
                    <button id="analyze-impact-btn" class="btn btn-outline">
                        <i class="icon-analyze"></i> 影响分析
                    </button>
                    <button id="save-weights-btn" class="btn btn-primary">
                        <i class="icon-save"></i> 保存配置
                    </button>
                </div>
            </div>
        `;

        this.updateWeightControls();
        this.renderWeightChart();
    }

    /**
     * 渲染单个权重控制器
     */
    renderWeightControl(indicator) {
        const config = this.coreIndicators[indicator];
        const currentWeight = this.currentWeights[indicator + '_weight'] || config.defaultWeight;
        
        return `
            <div class="weight-control" data-indicator="${indicator}">
                <div class="control-header">
                    <label class="control-label">
                        ${config.name}
                        <span class="weight-value" id="${indicator}-weight-display">${currentWeight.toFixed(1)}%</span>
                    </label>
                    <div class="control-description">${config.description}</div>
                </div>
                
                <div class="control-inputs">
                    <input 
                        type="range" 
                        id="${indicator}-weight-slider"
                        class="weight-slider"
                        min="${config.minWeight}" 
                        max="${config.maxWeight}" 
                        step="0.5"
                        value="${currentWeight}"
                    >
                    <input 
                        type="number" 
                        id="${indicator}-weight-input"
                        class="weight-input"
                        min="${config.minWeight}" 
                        max="${config.maxWeight}" 
                        step="0.5"
                        value="${currentWeight}"
                    >
                </div>
                
                <div class="control-range">
                    <span class="range-min">${config.minWeight}%</span>
                    <span class="range-max">${config.maxWeight}%</span>
                </div>
            </div>
        `;
    }

    /**
     * 更新权重值
     */
    updateWeight(indicator, value) {
        const config = this.coreIndicators[indicator];
        
        // 验证权重范围
        if (value < config.minWeight) value = config.minWeight;
        if (value > config.maxWeight) value = config.maxWeight;
        
        // 更新权重值
        this.currentWeights[indicator + '_weight'] = value;
        this.hasUnsavedChanges = true;
        
        // 更新界面显示
        this.updateWeightDisplay(indicator, value);
        this.updateWeightSummary();
        this.validateWeights();
        this.renderWeightChart();
        
        // 实时预览（如果启用）
        if (document.getElementById('auto-preview-checkbox')?.checked) {
            this.debouncePreview();
        }
    }

    /**
     * 更新权重显示
     */
    updateWeightDisplay(indicator, value) {
        const slider = document.getElementById(`${indicator}-weight-slider`);
        const input = document.getElementById(`${indicator}-weight-input`);
        const display = document.getElementById(`${indicator}-weight-display`);
        
        if (slider) slider.value = value;
        if (input) input.value = value;
        if (display) display.textContent = value.toFixed(1) + '%';
    }

    /**
     * 更新所有权重控制器
     */
    updateWeightControls() {
        Object.keys(this.coreIndicators).forEach(indicator => {
            const weight = this.currentWeights[indicator + '_weight'];
            if (weight !== undefined) {
                this.updateWeightDisplay(indicator, weight);
            }
        });
    }

    /**
     * 更新权重汇总
     */
    updateWeightSummary() {
        const total = Object.keys(this.coreIndicators).reduce((sum, indicator) => {
            return sum + (this.currentWeights[indicator + '_weight'] || 0);
        }, 0);
        
        const totalElement = document.getElementById('total-weight-value');
        if (totalElement) {
            totalElement.textContent = total.toFixed(1) + '%';
            totalElement.className = 'value ' + (Math.abs(total - 100) < 0.1 ? 'valid' : 'invalid');
        }
    }

    /**
     * 验证权重配置
     */
    validateWeights() {
        const total = Object.keys(this.coreIndicators).reduce((sum, indicator) => {
            return sum + (this.currentWeights[indicator + '_weight'] || 0);
        }, 0);
        
        const validationElement = document.getElementById('weight-validation');
        if (!validationElement) return;
        
        if (Math.abs(total - 100) < 0.1) {
            validationElement.innerHTML = `
                <div class="validation-success">
                    <i class="icon-check"></i> 权重配置有效
                </div>
            `;
        } else {
            validationElement.innerHTML = `
                <div class="validation-error">
                    <i class="icon-warning"></i> 权重总和必须为100%，当前为${total.toFixed(1)}%
                </div>
            `;
        }
    }

    /**
     * 渲染权重分布图表
     */
    renderWeightChart() {
        const chartContainer = document.getElementById('weight-chart');
        if (!chartContainer) return;

        const data = Object.keys(this.coreIndicators).map(indicator => ({
            name: this.coreIndicators[indicator].name,
            value: this.currentWeights[indicator + '_weight'] || 0,
            color: this.getIndicatorColor(indicator)
        }));

        const total = data.reduce((sum, item) => sum + item.value, 0);
        
        chartContainer.innerHTML = `
            <div class="chart-bars">
                ${data.map(item => `
                    <div class="chart-bar" style="--bar-width: ${(item.value / total * 100).toFixed(1)}%; --bar-color: ${item.color}">
                        <div class="bar-fill"></div>
                        <div class="bar-label">
                            <span class="bar-name">${item.name}</span>
                            <span class="bar-value">${item.value.toFixed(1)}%</span>
                        </div>
                    </div>
                `).join('')}
            </div>
        `;
    }

    /**
     * 获取指标颜色
     */
    getIndicatorColor(indicator) {
        const colors = {
            account_influence: '#3498db',
            cost_effectiveness: '#e74c3c',
            content_quality: '#2ecc71',
            medical_credibility: '#f39c12',
            roi_prediction: '#9b59b6'
        };
        return colors[indicator] || '#95a5a6';
    }

    /**
     * 应用预设方案
     */
    applyPresetScheme(schemeId) {
        if (!schemeId) return;
        
        const scheme = this.presetSchemes.find(s => s.id === schemeId);
        if (!scheme) return;
        
        // 更新权重值
        Object.keys(scheme.weights).forEach(key => {
            this.currentWeights[key] = scheme.weights[key];
        });
        
        // 更新界面
        this.updateWeightControls();
        this.updateWeightSummary();
        this.validateWeights();
        this.renderWeightChart();
        
        this.hasUnsavedChanges = true;
        
        // 显示方案信息
        this.app.showNotification(`已应用"${scheme.name}"权重方案`, 'success');
        
        // 显示适用场景
        this.showSchemeInfo(scheme);
    }

    /**
     * 显示方案信息
     */
    showSchemeInfo(scheme) {
        const modal = this.app.createModal('预设方案信息', `
            <div class="scheme-info">
                <h4>${scheme.name}</h4>
                <p class="scheme-description">${scheme.description}</p>
                
                <div class="scheme-weights">
                    <h5>权重配置:</h5>
                    <ul>
                        ${Object.keys(scheme.weights).map(key => {
                            const indicatorKey = key.replace('_weight', '');
                            const indicatorName = this.coreIndicators[indicatorKey]?.name || key;
                            return `<li>${indicatorName}: ${scheme.weights[key]}%</li>`;
                        }).join('')}
                    </ul>
                </div>
                
                <div class="scheme-scenarios">
                    <h5>适用场景:</h5>
                    <div class="scenario-tags">
                        ${scheme.suitable_scenarios.map(scenario => 
                            `<span class="scenario-tag">${scenario}</span>`
                        ).join('')}
                    </div>
                </div>
            </div>
        `);
        
        modal.show();
    }

    /**
     * 重置权重配置
     */
    resetWeights() {
        if (this.hasUnsavedChanges) {
            if (!confirm('确定要重置权重配置吗？未保存的更改将丢失。')) {
                return;
            }
        }
        
        this.currentWeights = this.getDefaultWeights();
        this.updateWeightControls();
        this.updateWeightSummary();
        this.validateWeights();
        this.renderWeightChart();
        
        this.hasUnsavedChanges = false;
        
        // 重置预设选择
        const presetSelect = document.getElementById('preset-scheme-select');
        if (presetSelect) presetSelect.value = '';
        
        this.app.showNotification('权重配置已重置为默认值', 'info');
    }

    /**
     * 显示保存权重配置模态框
     */
    showSaveWeightsModal() {
        if (!this.validateWeightsForSaving()) {
            return;
        }
        
        const modal = this.app.createModal('保存权重配置', `
            <form id="save-weights-form" class="save-weights-form">
                <div class="form-group">
                    <label for="config-name">配置名称 *</label>
                    <input type="text" id="config-name" class="form-input" required
                           placeholder="输入权重配置名称">
                </div>
                
                <div class="form-group">
                    <label for="config-description">配置说明</label>
                    <textarea id="config-description" class="form-textarea" rows="3"
                              placeholder="描述此权重配置的用途和特点（可选）"></textarea>
                </div>
                
                <div class="form-group">
                    <label class="checkbox-label">
                        <input type="checkbox" id="set-as-default" class="form-checkbox">
                        设为默认配置
                    </label>
                </div>
                
                <div class="current-weights-preview">
                    <h5>当前权重配置:</h5>
                    <div class="weights-list">
                        ${Object.keys(this.coreIndicators).map(indicator => {
                            const name = this.coreIndicators[indicator].name;
                            const weight = this.currentWeights[indicator + '_weight'];
                            return `<div class="weight-item">
                                <span class="weight-name">${name}:</span>
                                <span class="weight-value">${weight.toFixed(1)}%</span>
                            </div>`;
                        }).join('')}
                    </div>
                </div>
                
                <div class="form-actions">
                    <button type="button" class="btn btn-secondary" data-dismiss="modal">取消</button>
                    <button type="submit" class="btn btn-primary">保存配置</button>
                </div>
            </form>
        `);
        
        modal.show();
        
        // 设置表单提交事件
        const form = document.getElementById('save-weights-form');
        if (form) {
            form.addEventListener('submit', (e) => {
                e.preventDefault();
                this.saveWeightsConfig(modal);
            });
        }
    }

    /**
     * 验证权重配置是否可保存
     */
    validateWeightsForSaving() {
        const total = Object.keys(this.coreIndicators).reduce((sum, indicator) => {
            return sum + (this.currentWeights[indicator + '_weight'] || 0);
        }, 0);
        
        if (Math.abs(total - 100) >= 0.1) {
            this.app.showNotification('权重总和必须为100%才能保存', 'error');
            return false;
        }
        
        return true;
    }

    /**
     * 保存权重配置
     */
    async saveWeightsConfig(modal) {
        const form = document.getElementById('save-weights-form');
        const formData = new FormData(form);
        
        const configData = {
            name: formData.get('config-name'),
            description: formData.get('config-description') || '',
            account_influence_weight: this.currentWeights.account_influence_weight,
            cost_effectiveness_weight: this.currentWeights.cost_effectiveness_weight,
            content_quality_weight: this.currentWeights.content_quality_weight,
            medical_credibility_weight: this.currentWeights.medical_credibility_weight,
            roi_prediction_weight: this.currentWeights.roi_prediction_weight
        };
        
        try {
            const response = await fetch('/api/weight-configs/medical', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify(configData)
            });
            
            if (response.ok) {
                const result = await response.json();
                if (result.success) {
                    this.hasUnsavedChanges = false;
                    modal.hide();
                    this.app.showNotification('权重配置保存成功', 'success');
                      // 如果设为默认配置，激活该配置
                    if (document.getElementById('set-as-default')?.checked) {
                        await this.activateWeightConfig(result.data.id);
                        // 重新计算所有医生的医疗评分
                        await this.recalculateMedicalScores();
                    }
                } else {
                    throw new Error(result.message);
                }
            } else {
                throw new Error('保存请求失败');
            }
        } catch (error) {
            console.error('保存权重配置失败:', error);
            this.app.showNotification('保存失败: ' + error.message, 'error');
        }
    }    /**
     * 激活权重配置
     */
    async activateWeightConfig(configId) {
        try {
            const response = await fetch(`/api/weight-configs/${configId}/activate`, {
                method: 'POST'
            });
            
            if (response.ok) {
                this.app.showNotification('权重配置已激活', 'success');
            }
        } catch (error) {
            console.error('激活权重配置失败:', error);
        }
    }

    /**
     * 重新计算所有医生的医疗评分
     */
    async recalculateMedicalScores() {
        try {
            this.app.showNotification('正在重新计算医疗评分...', 'info');
            
            const response = await fetch('/api/medical-scoring/recalculate', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                }
            });
            
            if (response.ok) {
                const result = await response.json();
                if (result.success) {
                    this.app.showNotification(
                        `医疗评分重新计算完成，共处理 ${result.data.processed_count} 位医生`, 
                        'success'
                    );
                } else {
                    throw new Error(result.message);
                }
            } else {
                throw new Error('重新计算请求失败');
            }
        } catch (error) {
            console.error('重新计算医疗评分失败:', error);
            this.app.showNotification('重新计算失败: ' + error.message, 'error');
        }
    }

    /**
     * 权重影响分析
     */
    async analyzeWeightImpact() {
        if (!this.validateWeightsForSaving()) {
            return;
        }
        
        if (this.isAnalyzing) {
            this.app.showNotification('正在分析中，请稍候...', 'info');
            return;
        }
        
        this.isAnalyzing = true;
        
        const analyzeBtn = document.getElementById('analyze-impact-btn');
        if (analyzeBtn) {
            analyzeBtn.disabled = true;
            analyzeBtn.innerHTML = '<i class="icon-loading"></i> 分析中...';
        }
        
        try {
            const response = await fetch('/api/weight-configs/medical/analyze', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json'
                },
                body: JSON.stringify({
                    name: '临时分析配置',
                    account_influence_weight: this.currentWeights.account_influence_weight,
                    cost_effectiveness_weight: this.currentWeights.cost_effectiveness_weight,
                    content_quality_weight: this.currentWeights.content_quality_weight,
                    medical_credibility_weight: this.currentWeights.medical_credibility_weight,
                    roi_prediction_weight: this.currentWeights.roi_prediction_weight
                })
            });
            
            if (response.ok) {
                const result = await response.json();
                if (result.success) {
                    this.showAnalysisResults(result.data);
                } else {
                    throw new Error(result.message);
                }
            } else {
                throw new Error('分析请求失败');
            }
        } catch (error) {
            console.error('权重影响分析失败:', error);
            this.app.showNotification('分析失败: ' + error.message, 'error');
        } finally {
            this.isAnalyzing = false;
            if (analyzeBtn) {
                analyzeBtn.disabled = false;
                analyzeBtn.innerHTML = '<i class="icon-analyze"></i> 影响分析';
            }
        }
    }

    /**
     * 显示分析结果
     */
    showAnalysisResults(analysisData) {
        const analysisPanel = document.getElementById('analysis-panel');
        const analysisContent = document.getElementById('analysis-content');
        
        if (!analysisPanel || !analysisContent) return;
        
        analysisContent.innerHTML = `
            <div class="analysis-results">
                <!-- 样本医生分析 -->
                <div class="sample-analysis">
                    <h5>样本医生评分预测</h5>
                    <div class="doctor-sample">
                        <div class="doctor-info">
                            <h6>${analysisData.sample_analysis.doctor.name}</h6>
                            <div class="predicted-score">
                                预测综合评分: <span class="score-value">${analysisData.sample_analysis.predicted_score}</span>
                            </div>
                        </div>
                        
                        <div class="score-breakdown">
                            <h6>评分构成:</h6>
                            <div class="breakdown-list">
                                <div class="breakdown-item">
                                    <span class="item-name">账号影响力贡献:</span>
                                    <span class="item-value">${analysisData.sample_analysis.score_breakdown.account_influence_contribution}</span>
                                </div>
                                <div class="breakdown-item">
                                    <span class="item-name">性价比贡献:</span>
                                    <span class="item-value">${analysisData.sample_analysis.score_breakdown.cost_effectiveness_contribution}</span>
                                </div>
                                <div class="breakdown-item">
                                    <span class="item-name">内容质量贡献:</span>
                                    <span class="item-value">${analysisData.sample_analysis.score_breakdown.content_quality_contribution}</span>
                                </div>
                                <div class="breakdown-item">
                                    <span class="item-name">医疗可信度贡献:</span>
                                    <span class="item-value">${analysisData.sample_analysis.score_breakdown.medical_credibility_contribution}</span>
                                </div>
                                <div class="breakdown-item">
                                    <span class="item-name">ROI预测贡献:</span>
                                    <span class="item-value">${analysisData.sample_analysis.score_breakdown.roi_prediction_contribution}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                
                <!-- 策略分析 -->
                <div class="strategy-analysis">
                    <h5>权重策略分析</h5>
                    <div class="strategy-description">
                        ${analysisData.strategy_analysis}
                    </div>
                    
                    <div class="weight-distribution">
                        <div class="distribution-item">
                            <span class="label">主导因素:</span>
                            <span class="value">${this.getFactorName(analysisData.weight_distribution.dominant_factor)}</span>
                        </div>
                        <div class="distribution-item">
                            <span class="label">平衡度评分:</span>
                            <span class="value">${analysisData.weight_distribution.balance_score.toFixed(1)}</span>
                        </div>
                        <div class="distribution-item">
                            <span class="label">风险评估:</span>
                            <span class="value risk-${this.getRiskLevel(analysisData.weight_distribution.risk_level)}">${analysisData.weight_distribution.risk_level}</span>
                        </div>
                    </div>
                </div>
                
                <!-- 优化建议 -->
                <div class="recommendations">
                    <h5>优化建议</h5>
                    <ul class="recommendation-list">
                        ${analysisData.recommendations.map(rec => `<li>${rec}</li>`).join('')}
                    </ul>
                </div>
            </div>
        `;
        
        analysisPanel.style.display = 'block';
        
        // 设置关闭按钮事件
        const closeBtn = document.getElementById('close-analysis-btn');
        if (closeBtn) {
            closeBtn.addEventListener('click', () => {
                analysisPanel.style.display = 'none';
            });
        }
    }

    /**
     * 获取因素名称
     */
    getFactorName(factor) {
        const factorNames = {
            account_influence: '账号影响力',
            cost_effectiveness: '性价比',
            content_quality: '内容质量',
            medical_credibility: '医疗可信度',
            roi_prediction: 'ROI预测'
        };
        return factorNames[factor] || factor;
    }

    /**
     * 获取风险等级
     */
    getRiskLevel(riskText) {
        if (riskText.includes('高风险')) return 'high';
        if (riskText.includes('中风险')) return 'medium';
        return 'low';
    }

    /**
     * 实时预览权重影响
     */
    previewWeightImpact() {
        // 实现实时预览功能
        this.app.showNotification('实时预览功能开发中...', 'info');
    }

    /**
     * 防抖预览
     */
    debouncePreview() {
        clearTimeout(this.previewTimer);
        this.previewTimer = setTimeout(() => {
            this.previewWeightImpact();
        }, 500);
    }
}

// 导出模块
window.MedicalWeightsModule = MedicalWeightsModule;
