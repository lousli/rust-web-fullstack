/**
 * 数据导入功能模块
 * 负责CSV文件导入、数据验证和批量处理
 */
class ImportModule {
    constructor(app) {
        this.app = app;
        this.importData = [];
        this.validationResults = null;
        this.importStep = 1; // 1: 文件上传, 2: 数据预览, 3: 导入结果
    }

    /**
     * 初始化导入模块
     */
    async init() {
        this.setupEventListeners();
        this.renderImportInterface();
    }

    /**
     * 设置事件监听器
     */
    setupEventListeners() {
        // 文件上传
        const fileInput = document.getElementById('csv-file-input');
        if (fileInput) {
            fileInput.addEventListener('change', (e) => {
                this.handleFileSelect(e.target.files[0]);
            });
        }

        // 拖拽上传
        const dropZone = document.getElementById('csv-drop-zone');
        if (dropZone) {
            dropZone.addEventListener('dragover', (e) => {
                e.preventDefault();
                dropZone.classList.add('drag-over');
            });

            dropZone.addEventListener('dragleave', (e) => {
                e.preventDefault();
                dropZone.classList.remove('drag-over');
            });

            dropZone.addEventListener('drop', (e) => {
                e.preventDefault();
                dropZone.classList.remove('drag-over');
                const file = e.dataTransfer.files[0];
                if (file) {
                    this.handleFileSelect(file);
                }
            });
        }

        // 下载模板
        const downloadTemplateBtn = document.getElementById('download-template-btn');
        if (downloadTemplateBtn) {
            downloadTemplateBtn.addEventListener('click', () => {
                this.downloadTemplate();
            });
        }

        // 确认导入
        const confirmImportBtn = document.getElementById('confirm-import-btn');
        if (confirmImportBtn) {
            confirmImportBtn.addEventListener('click', () => {
                this.confirmImport();
            });
        }

        // 重新开始
        const restartBtn = document.getElementById('restart-import-btn');
        if (restartBtn) {
            restartBtn.addEventListener('click', () => {
                this.restartImport();
            });
        }
    }

    /**
     * 渲染导入界面
     */
    renderImportInterface() {
        const container = document.getElementById('import-content');
        if (!container) return;

        switch (this.importStep) {
            case 1:
                this.renderFileUpload();
                break;
            case 2:
                this.renderDataPreview();
                break;
            case 3:
                this.renderImportResults();
                break;
        }
    }

    /**
     * 渲染文件上传界面
     */
    renderFileUpload() {
        const container = document.getElementById('import-content');
        
        container.innerHTML = `
            <div class="import-upload">
                <div class="upload-section">
                    <div class="section-header">
                        <h3>上传CSV文件</h3>
                        <p>支持批量导入医生数据，请确保CSV文件格式正确</p>
                    </div>
                    
                    <div class="upload-area">
                        <div class="drop-zone" id="csv-drop-zone">
                            <div class="drop-zone-content">
                                <div class="upload-icon">📁</div>
                                <div class="upload-title">拖拽CSV文件到此处</div>
                                <div class="upload-subtitle">或点击下方按钮选择文件</div>
                                <input type="file" id="csv-file-input" accept=".csv" style="display: none;">
                                <button class="btn btn-primary" onclick="document.getElementById('csv-file-input').click()">
                                    选择文件
                                </button>
                            </div>
                        </div>
                    </div>
                    
                    <div class="upload-help">
                        <div class="help-section">
                            <h4>文件格式要求</h4>
                            <ul>
                                <li>文件格式：CSV (逗号分隔值)</li>
                                <li>字符编码：UTF-8</li>
                                <li>包含表头行</li>
                                <li>文件大小：不超过10MB</li>
                            </ul>
                        </div>
                        
                        <div class="help-section">
                            <h4>必填字段</h4>
                            <div class="required-fields">
                                <span class="field-tag">姓名</span>
                                <span class="field-tag">职称</span>
                                <span class="field-tag">医院</span>
                                <span class="field-tag">科室</span>
                            </div>
                        </div>
                        
                        <div class="help-section">
                            <h4>可选字段</h4>
                            <div class="optional-fields">
                                <span class="field-tag">粉丝数</span>
                                <span class="field-tag">文章数</span>
                                <span class="field-tag">平均阅读量</span>
                                <span class="field-tag">平均点赞数</span>
                                <span class="field-tag">月发文量</span>
                                <span class="field-tag">回复率</span>
                            </div>
                        </div>
                    </div>
                    
                    <div class="template-section">
                        <h4>需要模板？</h4>
                        <p>下载标准CSV模板，按照模板格式准备数据</p>
                        <button id="download-template-btn" class="btn btn-outline">
                            <span class="btn-icon">⬇️</span>
                            下载CSV模板
                        </button>
                    </div>
                </div>
            </div>
        `;

        this.bindFileUploadEvents();
    }

    /**
     * 绑定文件上传事件
     */
    bindFileUploadEvents() {
        // 重新绑定事件监听器
        const fileInput = document.getElementById('csv-file-input');
        if (fileInput) {
            fileInput.addEventListener('change', (e) => {
                this.handleFileSelect(e.target.files[0]);
            });
        }

        const dropZone = document.getElementById('csv-drop-zone');
        if (dropZone) {
            dropZone.addEventListener('dragover', (e) => {
                e.preventDefault();
                dropZone.classList.add('drag-over');
            });

            dropZone.addEventListener('dragleave', (e) => {
                e.preventDefault();
                dropZone.classList.remove('drag-over');
            });

            dropZone.addEventListener('drop', (e) => {
                e.preventDefault();
                dropZone.classList.remove('drag-over');
                const file = e.dataTransfer.files[0];
                if (file) {
                    this.handleFileSelect(file);
                }
            });
        }

        const downloadTemplateBtn = document.getElementById('download-template-btn');
        if (downloadTemplateBtn) {
            downloadTemplateBtn.addEventListener('click', () => {
                this.downloadTemplate();
            });
        }
    }

    /**
     * 处理文件选择
     */
    async handleFileSelect(file) {
        if (!file) return;

        // 验证文件类型
        if (!file.name.toLowerCase().endsWith('.csv')) {
            this.app.showNotification('请选择CSV文件', 'error');
            return;
        }

        // 验证文件大小
        if (file.size > 10 * 1024 * 1024) {
            this.app.showNotification('文件大小不能超过10MB', 'error');
            return;
        }

        try {
            this.app.showLoading(true);
            
            // 读取文件内容
            const text = await this.readFileAsText(file);
            
            // 解析CSV
            this.importData = this.parseCSV(text);
            
            // 验证数据
            this.validationResults = this.validateData(this.importData);
            
            // 切换到预览步骤
            this.importStep = 2;
            this.renderImportInterface();
            
        } catch (error) {
            console.error('处理文件失败:', error);
            this.app.showNotification('处理文件失败: ' + error.message, 'error');
        } finally {
            this.app.showLoading(false);
        }
    }

    /**
     * 读取文件为文本
     */
    readFileAsText(file) {
        return new Promise((resolve, reject) => {
            const reader = new FileReader();
            reader.onload = (e) => resolve(e.target.result);
            reader.onerror = (e) => reject(new Error('文件读取失败'));
            reader.readAsText(file, 'UTF-8');
        });
    }

    /**
     * 解析CSV
     */
    parseCSV(text) {
        const lines = text.split('\n').filter(line => line.trim());
        if (lines.length === 0) {
            throw new Error('CSV文件为空');
        }

        const headers = this.parseCSVLine(lines[0]);
        const data = [];

        for (let i = 1; i < lines.length; i++) {
            const values = this.parseCSVLine(lines[i]);
            if (values.length > 0) {
                const row = {};
                headers.forEach((header, index) => {
                    row[header.trim()] = values[index] ? values[index].trim() : '';
                });
                data.push(row);
            }
        }

        return data;
    }

    /**
     * 解析CSV行
     */
    parseCSVLine(line) {
        const result = [];
        let current = '';
        let inQuotes = false;

        for (let i = 0; i < line.length; i++) {
            const char = line[i];
            const nextChar = line[i + 1];

            if (char === '"' && inQuotes && nextChar === '"') {
                current += '"';
                i++; // 跳过下一个引号
            } else if (char === '"') {
                inQuotes = !inQuotes;
            } else if (char === ',' && !inQuotes) {
                result.push(current);
                current = '';
            } else {
                current += char;
            }
        }

        result.push(current);
        return result;
    }

    /**
     * 验证数据
     */
    validateData(data) {
        const results = {
            valid: [],
            invalid: [],
            warnings: [],
            summary: {
                total: data.length,
                validCount: 0,
                invalidCount: 0,
                warningCount: 0
            }
        };

        // 检查必填字段映射
        const fieldMapping = this.getFieldMapping(data[0] || {});
        
        data.forEach((row, index) => {
            const validation = this.validateRow(row, fieldMapping, index + 1);
            
            if (validation.isValid) {
                results.valid.push({
                    rowIndex: index + 1,
                    data: validation.normalizedData,
                    warnings: validation.warnings
                });
                results.summary.validCount++;
                
                if (validation.warnings.length > 0) {
                    results.summary.warningCount++;
                }
            } else {
                results.invalid.push({
                    rowIndex: index + 1,
                    data: row,
                    errors: validation.errors
                });
                results.summary.invalidCount++;
            }
        });

        return results;
    }

    /**
     * 获取字段映射
     */
    getFieldMapping(firstRow) {
        const headers = Object.keys(firstRow);
        const mapping = {};

        // 中文字段映射
        const fieldMap = {
            '姓名': 'name',
            '医生姓名': 'name',
            'name': 'name',
            '职称': 'title',
            '医生职称': 'title',
            'title': 'title',
            '医院': 'hospital',
            '医院名称': 'hospital',
            'hospital': 'hospital',
            '科室': 'department',
            '科室名称': 'department',
            'department': 'department',
            '粉丝数': 'followers_count',
            '粉丝数量': 'followers_count',
            'followers': 'followers_count',
            'followers_count': 'followers_count',
            '文章数': 'articles_count',
            '文章数量': 'articles_count',
            'articles': 'articles_count',
            'articles_count': 'articles_count',
            '平均阅读量': 'avg_views',
            '阅读量': 'avg_views',
            'views': 'avg_views',
            'avg_views': 'avg_views',
            '平均点赞数': 'avg_likes',
            '点赞数': 'avg_likes',
            'likes': 'avg_likes',
            'avg_likes': 'avg_likes',
            '月发文量': 'monthly_articles',
            '月发文数': 'monthly_articles',
            'monthly_articles': 'monthly_articles',
            '回复率': 'response_rate',
            '响应率': 'response_rate',
            'response_rate': 'response_rate'
        };

        headers.forEach(header => {
            const normalizedHeader = header.trim();
            if (fieldMap[normalizedHeader]) {
                mapping[normalizedHeader] = fieldMap[normalizedHeader];
            }
        });

        return mapping;
    }

    /**
     * 验证行数据
     */
    validateRow(row, fieldMapping, rowIndex) {
        const errors = [];
        const warnings = [];
        const normalizedData = {};

        // 验证必填字段
        const requiredFields = ['name', 'title', 'hospital', 'department'];
        
        for (const [csvField, dbField] of Object.entries(fieldMapping)) {
            const value = row[csvField];
            
            if (requiredFields.includes(dbField)) {
                if (!value || value.trim() === '') {
                    errors.push(`第${rowIndex}行：${csvField} 是必填字段`);
                    continue;
                }
            }

            // 数据类型验证和转换
            try {
                normalizedData[dbField] = this.normalizeFieldValue(dbField, value);
            } catch (error) {
                if (requiredFields.includes(dbField)) {
                    errors.push(`第${rowIndex}行：${csvField} 格式错误 - ${error.message}`);
                } else {
                    warnings.push(`第${rowIndex}行：${csvField} 格式错误，将使用默认值`);
                    normalizedData[dbField] = this.getDefaultValue(dbField);
                }
            }
        }

        // 特殊验证
        if (normalizedData.title) {
            const validTitles = ['主任医师', '副主任医师', '主治医师', '住院医师'];
            if (!validTitles.includes(normalizedData.title)) {
                warnings.push(`第${rowIndex}行：职称 "${normalizedData.title}" 不在标准列表中`);
            }
        }

        if (normalizedData.response_rate && (normalizedData.response_rate < 0 || normalizedData.response_rate > 100)) {
            warnings.push(`第${rowIndex}行：回复率应在0-100之间`);
            normalizedData.response_rate = Math.max(0, Math.min(100, normalizedData.response_rate));
        }

        return {
            isValid: errors.length === 0,
            errors,
            warnings,
            normalizedData
        };
    }

    /**
     * 标准化字段值
     */
    normalizeFieldValue(field, value) {
        if (!value || value.trim() === '') {
            return this.getDefaultValue(field);
        }

        const trimmedValue = value.trim();

        switch (field) {
            case 'name':
            case 'title':
            case 'hospital':
            case 'department':
                return trimmedValue;
            
            case 'followers_count':
            case 'articles_count':
            case 'avg_views':
            case 'avg_likes':
            case 'monthly_articles':
                const intValue = parseInt(trimmedValue.replace(/[,，]/g, ''));
                if (isNaN(intValue) || intValue < 0) {
                    throw new Error('必须是非负整数');
                }
                return intValue;
            
            case 'response_rate':
                const floatValue = parseFloat(trimmedValue.replace('%', ''));
                if (isNaN(floatValue)) {
                    throw new Error('必须是数字');
                }
                return floatValue;
            
            default:
                return trimmedValue;
        }
    }

    /**
     * 获取默认值
     */
    getDefaultValue(field) {
        const defaults = {
            'followers_count': 0,
            'articles_count': 0,
            'avg_views': 0,
            'avg_likes': 0,
            'monthly_articles': 0,
            'response_rate': 0.0
        };
        return defaults[field] || '';
    }

    /**
     * 渲染数据预览
     */
    renderDataPreview() {
        const container = document.getElementById('import-content');
        
        container.innerHTML = `
            <div class="import-preview">
                <div class="preview-header">
                    <h3>数据预览</h3>
                    <div class="validation-summary">
                        <div class="summary-item valid">
                            <span class="count">${this.validationResults.summary.validCount}</span>
                            <span class="label">有效记录</span>
                        </div>
                        <div class="summary-item invalid">
                            <span class="count">${this.validationResults.summary.invalidCount}</span>
                            <span class="label">无效记录</span>
                        </div>
                        <div class="summary-item warning">
                            <span class="count">${this.validationResults.summary.warningCount}</span>
                            <span class="label">警告记录</span>
                        </div>
                    </div>
                </div>
                
                <div class="preview-tabs">
                    <button class="tab-button active" data-tab="valid">
                        有效数据 (${this.validationResults.summary.validCount})
                    </button>
                    <button class="tab-button" data-tab="invalid">
                        错误数据 (${this.validationResults.summary.invalidCount})
                    </button>
                    <button class="tab-button" data-tab="warnings">
                        警告数据 (${this.validationResults.summary.warningCount})
                    </button>
                </div>
                
                <div class="preview-content">
                    <div class="tab-panel active" id="valid-panel">
                        ${this.renderValidDataTable()}
                    </div>
                    <div class="tab-panel" id="invalid-panel">
                        ${this.renderInvalidDataTable()}
                    </div>
                    <div class="tab-panel" id="warnings-panel">
                        ${this.renderWarningsTable()}
                    </div>
                </div>
                
                <div class="preview-actions">
                    <button class="btn btn-outline" onclick="importModule.restartImport()">
                        重新选择文件
                    </button>
                    <button id="confirm-import-btn" class="btn btn-primary" 
                            ${this.validationResults.summary.validCount === 0 ? 'disabled' : ''}>
                        确认导入 (${this.validationResults.summary.validCount} 条记录)
                    </button>
                </div>
            </div>
        `;

        this.bindPreviewEvents();
    }

    /**
     * 渲染有效数据表格
     */
    renderValidDataTable() {
        if (this.validationResults.valid.length === 0) {
            return '<div class="empty-state">没有有效的数据记录</div>';
        }

        const headers = ['行号', '姓名', '职称', '医院', '科室', '粉丝数', '文章数', '警告'];
        const rows = this.validationResults.valid.slice(0, 50); // 最多显示50条

        return `
            <div class="data-table">
                <table class="preview-table">
                    <thead>
                        <tr>
                            ${headers.map(header => `<th>${header}</th>`).join('')}
                        </tr>
                    </thead>
                    <tbody>
                        ${rows.map(item => `
                            <tr>
                                <td>${item.rowIndex}</td>
                                <td>${item.data.name || '-'}</td>
                                <td>${item.data.title || '-'}</td>
                                <td>${item.data.hospital || '-'}</td>
                                <td>${item.data.department || '-'}</td>
                                <td>${this.app.formatNumber(item.data.followers_count)}</td>
                                <td>${this.app.formatNumber(item.data.articles_count)}</td>
                                <td>
                                    ${item.warnings.length > 0 ? 
                                        `<span class="warning-badge" title="${item.warnings.join('; ')}">${item.warnings.length}个警告</span>` : 
                                        '-'}
                                </td>
                            </tr>
                        `).join('')}
                    </tbody>
                </table>
                ${this.validationResults.valid.length > 50 ? 
                    `<div class="table-note">显示前50条记录，共${this.validationResults.valid.length}条</div>` : 
                    ''}
            </div>
        `;
    }

    /**
     * 渲染无效数据表格
     */
    renderInvalidDataTable() {
        if (this.validationResults.invalid.length === 0) {
            return '<div class="empty-state">没有无效的数据记录</div>';
        }

        return `
            <div class="error-list">
                ${this.validationResults.invalid.map(item => `
                    <div class="error-item">
                        <div class="error-header">
                            <span class="row-number">第 ${item.rowIndex} 行</span>
                            <span class="error-count">${item.errors.length} 个错误</span>
                        </div>
                        <div class="error-data">
                            ${Object.entries(item.data).map(([key, value]) => 
                                `<span class="data-field">${key}: ${value}</span>`
                            ).join('')}
                        </div>
                        <div class="error-messages">
                            ${item.errors.map(error => 
                                `<div class="error-message">❌ ${error}</div>`
                            ).join('')}
                        </div>
                    </div>
                `).join('')}
            </div>
        `;
    }

    /**
     * 渲染警告数据表格
     */
    renderWarningsTable() {
        const warningItems = this.validationResults.valid.filter(item => item.warnings.length > 0);
        
        if (warningItems.length === 0) {
            return '<div class="empty-state">没有警告记录</div>';
        }

        return `
            <div class="warning-list">
                ${warningItems.map(item => `
                    <div class="warning-item">
                        <div class="warning-header">
                            <span class="row-number">第 ${item.rowIndex} 行</span>
                            <span class="doctor-name">${item.data.name}</span>
                            <span class="warning-count">${item.warnings.length} 个警告</span>
                        </div>
                        <div class="warning-messages">
                            ${item.warnings.map(warning => 
                                `<div class="warning-message">⚠️ ${warning}</div>`
                            ).join('')}
                        </div>
                    </div>
                `).join('')}
            </div>
        `;
    }

    /**
     * 绑定预览事件
     */
    bindPreviewEvents() {
        // 标签页切换
        document.querySelectorAll('.tab-button').forEach(button => {
            button.addEventListener('click', (e) => {
                const tab = e.target.dataset.tab;
                this.switchPreviewTab(tab);
            });
        });

        // 确认导入
        const confirmBtn = document.getElementById('confirm-import-btn');
        if (confirmBtn) {
            confirmBtn.addEventListener('click', () => {
                this.confirmImport();
            });
        }
    }

    /**
     * 切换预览标签
     */
    switchPreviewTab(tab) {
        // 更新标签按钮状态
        document.querySelectorAll('.tab-button').forEach(button => {
            button.classList.toggle('active', button.dataset.tab === tab);
        });

        // 更新面板显示
        document.querySelectorAll('.tab-panel').forEach(panel => {
            panel.classList.remove('active');
        });
        
        const targetPanel = document.getElementById(`${tab}-panel`);
        if (targetPanel) {
            targetPanel.classList.add('active');
        }
    }

    /**
     * 确认导入
     */
    async confirmImport() {
        if (this.validationResults.summary.validCount === 0) {
            this.app.showNotification('没有有效数据可以导入', 'error');
            return;
        }

        const confirmMessage = `确定要导入 ${this.validationResults.summary.validCount} 条医生记录吗？`;
        if (this.validationResults.summary.warningCount > 0) {
            confirmMessage += `\n注意：有 ${this.validationResults.summary.warningCount} 条记录包含警告。`;
        }

        if (!confirm(confirmMessage)) {
            return;
        }

        try {
            this.app.showLoading(true);

            // 批量导入数据
            const importResults = await this.performImport();
            
            // 切换到结果步骤
            this.importStep = 3;
            this.importResults = importResults;
            this.renderImportInterface();

        } catch (error) {
            console.error('导入失败:', error);
            this.app.showNotification('导入失败: ' + error.message, 'error');
        } finally {
            this.app.showLoading(false);
        }
    }

    /**
     * 执行导入
     */
    async performImport() {
        const results = {
            success: [],
            failed: [],
            summary: {
                total: this.validationResults.valid.length,
                successCount: 0,
                failedCount: 0
            }
        };

        // 批量处理，每次处理10条记录
        const batchSize = 10;
        const batches = [];
        
        for (let i = 0; i < this.validationResults.valid.length; i += batchSize) {
            batches.push(this.validationResults.valid.slice(i, i + batchSize));
        }

        for (const batch of batches) {
            const batchPromises = batch.map(async (item) => {
                try {
                    await this.app.api.createDoctor(item.data);
                    results.success.push({
                        rowIndex: item.rowIndex,
                        name: item.data.name
                    });
                    results.summary.successCount++;
                } catch (error) {
                    results.failed.push({
                        rowIndex: item.rowIndex,
                        name: item.data.name,
                        error: error.message
                    });
                    results.summary.failedCount++;
                }
            });

            await Promise.all(batchPromises);
        }

        return results;
    }

    /**
     * 渲染导入结果
     */
    renderImportResults() {
        const container = document.getElementById('import-content');
        
        container.innerHTML = `
            <div class="import-results">
                <div class="results-header">
                    <div class="success-icon">✅</div>
                    <h3>导入完成</h3>
                    <div class="results-summary">
                        <div class="summary-item success">
                            <span class="count">${this.importResults.summary.successCount}</span>
                            <span class="label">成功导入</span>
                        </div>
                        <div class="summary-item failed">
                            <span class="count">${this.importResults.summary.failedCount}</span>
                            <span class="label">导入失败</span>
                        </div>
                    </div>
                </div>

                ${this.importResults.summary.failedCount > 0 ? `
                    <div class="failed-section">
                        <h4>导入失败的记录</h4>
                        <div class="failed-list">
                            ${this.importResults.failed.map(item => `
                                <div class="failed-item">
                                    <span class="row-info">第${item.rowIndex}行 - ${item.name}</span>
                                    <span class="error-info">${item.error}</span>
                                </div>
                            `).join('')}
                        </div>
                    </div>
                ` : ''}

                <div class="results-actions">
                    <button id="restart-import-btn" class="btn btn-outline">
                        重新导入
                    </button>
                    <button class="btn btn-primary" onclick="app.navigate('doctors')">
                        查看医生列表
                    </button>
                </div>
            </div>
        `;

        this.bindResultsEvents();
    }

    /**
     * 绑定结果事件
     */
    bindResultsEvents() {
        const restartBtn = document.getElementById('restart-import-btn');
        if (restartBtn) {
            restartBtn.addEventListener('click', () => {
                this.restartImport();
            });
        }
    }

    /**
     * 重新开始导入
     */
    restartImport() {
        this.importData = [];
        this.validationResults = null;
        this.importResults = null;
        this.importStep = 1;
        this.renderImportInterface();
    }

    /**
     * 下载CSV模板
     */
    downloadTemplate() {
        const headers = [
            '姓名', '职称', '医院', '科室', '粉丝数', '文章数', 
            '平均阅读量', '平均点赞数', '月发文量', '回复率'
        ];
        
        const sampleData = [
            ['张医生', '主任医师', '北京协和医院', '心内科', '50000', '120', '2000', '150', '8', '85.5'],
            ['李医生', '副主任医师', '上海华山医院', '神经内科', '30000', '80', '1500', '100', '6', '78.2'],
            ['王医生', '主治医师', '广州中山医院', '消化内科', '20000', '60', '1000', '80', '4', '72.8']
        ];

        const csvContent = [headers, ...sampleData]
            .map(row => row.join(','))
            .join('\n');

        const blob = new Blob(['\ufeff' + csvContent], { type: 'text/csv;charset=utf-8;' });
        const link = document.createElement('a');
        link.href = URL.createObjectURL(blob);
        link.download = 'doctor_import_template.csv';
        link.click();
        URL.revokeObjectURL(link.href);

        this.app.showNotification('模板下载成功', 'success');
    }
}

// 导出模块
if (typeof module !== 'undefined' && module.exports) {
    module.exports = ImportModule;
} else {
    window.ImportModule = ImportModule;
}
