/**
 * 医生管理功能模块
 * 负责医生信息的增删改查、列表展示和搜索功能
 */
class DoctorsModule {
    constructor(app) {
        this.app = app;
        this.currentPage = 1;
        this.pageSize = 20;
        this.totalPages = 1;
        this.currentFilter = {};
        this.currentSort = { field: 'name', order: 'asc' };
        this.doctors = [];
    }

    /**
     * 初始化医生管理模块
     */
    async init() {
        this.setupEventListeners();
        await this.loadDoctors();
        this.renderDoctorsList();
    }

    /**
     * 设置事件监听器
     */
    setupEventListeners() {
        // 搜索功能
        const searchInput = document.getElementById('doctor-search');
        if (searchInput) {
            let searchTimeout;
            searchInput.addEventListener('input', (e) => {
                clearTimeout(searchTimeout);
                searchTimeout = setTimeout(() => {
                    this.handleSearch(e.target.value);
                }, 300);
            });
        }

        // 筛选功能
        const departmentFilter = document.getElementById('department-filter');
        if (departmentFilter) {
            departmentFilter.addEventListener('change', (e) => {
                this.handleFilter('department', e.target.value);
            });
        }

        const titleFilter = document.getElementById('title-filter');
        if (titleFilter) {
            titleFilter.addEventListener('change', (e) => {
                this.handleFilter('title', e.target.value);
            });
        }

        // 添加医生按钮
        const addDoctorBtn = document.getElementById('add-doctor-btn');
        if (addDoctorBtn) {
            addDoctorBtn.addEventListener('click', () => {
                this.showDoctorModal();
            });
        }

        // 批量操作
        const selectAllCheckbox = document.getElementById('select-all-doctors');
        if (selectAllCheckbox) {
            selectAllCheckbox.addEventListener('change', (e) => {
                this.handleSelectAll(e.target.checked);
            });
        }

        const deleteSelectedBtn = document.getElementById('delete-selected-btn');
        if (deleteSelectedBtn) {
            deleteSelectedBtn.addEventListener('click', () => {
                this.handleBatchDelete();
            });
        }
    }

    /**
     * 加载医生数据
     */
    async loadDoctors() {
        try {
            this.app.showLoading(true);
            
            const params = {
                page: this.currentPage,
                page_size: this.pageSize,
                ...this.currentFilter,
                sort_by: this.currentSort.field,
                order: this.currentSort.order
            };

            const response = await this.app.api.getDoctors(params);
            this.doctors = response.doctors || response || [];
            this.totalPages = Math.ceil((response.total || this.doctors.length) / this.pageSize);
            
        } catch (error) {
            console.error('加载医生数据失败:', error);
            this.app.showNotification('加载医生数据失败', 'error');
            this.doctors = [];
        } finally {
            this.app.showLoading(false);
        }
    }

    /**
     * 渲染医生列表
     */
    renderDoctorsList() {
        const listContainer = document.getElementById('doctors-list');
        if (!listContainer) return;

        if (this.doctors.length === 0) {
            listContainer.innerHTML = `
                <div class="empty-state">
                    <div class="empty-icon">👨‍⚕️</div>
                    <div class="empty-title">暂无医生数据</div>
                    <div class="empty-description">点击"添加医生"按钮开始添加医生信息</div>
                    <button class="btn btn-primary" onclick="doctorsModule.showDoctorModal()">
                        添加医生
                    </button>
                </div>
            `;
            return;
        }

        listContainer.innerHTML = `
            <div class="doctors-table">
                <div class="table-header">
                    <div class="table-cell">
                        <input type="checkbox" id="select-all-doctors">
                    </div>
                    <div class="table-cell sortable" data-field="name">
                        姓名 ${this.getSortIcon('name')}
                    </div>
                    <div class="table-cell sortable" data-field="title">
                        职称 ${this.getSortIcon('title')}
                    </div>
                    <div class="table-cell sortable" data-field="department">
                        科室 ${this.getSortIcon('department')}
                    </div>
                    <div class="table-cell sortable" data-field="total_score">
                        总分 ${this.getSortIcon('total_score')}
                    </div>
                    <div class="table-cell">投放建议</div>
                    <div class="table-cell">操作</div>
                </div>
                <div class="table-body">
                    ${this.doctors.map(doctor => this.renderDoctorRow(doctor)).join('')}
                </div>
            </div>
            ${this.renderPagination()}
        `;

        this.bindTableEvents();
    }

    /**
     * 渲染医生行
     */
    renderDoctorRow(doctor) {
        return `
            <div class="table-row" data-doctor-id="${doctor.id}">
                <div class="table-cell">
                    <input type="checkbox" class="doctor-checkbox" value="${doctor.id}">
                </div>
                <div class="table-cell">
                    <div class="doctor-name">${doctor.name}</div>
                    <div class="doctor-hospital">${doctor.hospital}</div>
                </div>
                <div class="table-cell">
                    <span class="title-badge ${this.getTitleClass(doctor.title)}">${doctor.title}</span>
                </div>
                <div class="table-cell">${doctor.department}</div>
                <div class="table-cell">
                    <div class="score-display">
                        <div class="total-score">${this.app.formatNumber(doctor.total_score)}</div>
                        <div class="score-breakdown">
                            影响力: ${this.app.formatNumber(doctor.influence_score)} | 
                            质量: ${this.app.formatNumber(doctor.quality_score)} | 
                            活跃度: ${this.app.formatNumber(doctor.activity_score)}
                        </div>
                    </div>
                </div>
                <div class="table-cell">
                    <span class="investment-badge ${this.getInvestmentClass(doctor.recommendation)}">
                        ${doctor.recommendation || '待评估'}
                    </span>
                </div>
                <div class="table-cell">
                    <div class="action-buttons">
                        <button class="btn btn-sm btn-outline" onclick="doctorsModule.viewDoctor(${doctor.id})">
                            查看
                        </button>
                        <button class="btn btn-sm btn-outline" onclick="doctorsModule.editDoctor(${doctor.id})">
                            编辑
                        </button>
                        <button class="btn btn-sm btn-danger" onclick="doctorsModule.deleteDoctor(${doctor.id})">
                            删除
                        </button>
                    </div>
                </div>
            </div>
        `;
    }

    /**
     * 获取职称样式类名
     */
    getTitleClass(title) {
        const titleMap = {
            '主任医师': 'title-senior',
            '副主任医师': 'title-associate',
            '主治医师': 'title-attending',
            '住院医师': 'title-resident'
        };
        return titleMap[title] || 'title-other';
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
     * 获取排序图标
     */
    getSortIcon(field) {
        if (this.currentSort.field !== field) {
            return '<span class="sort-icon">⇅</span>';
        }
        return this.currentSort.order === 'asc' 
            ? '<span class="sort-icon active">↑</span>' 
            : '<span class="sort-icon active">↓</span>';
    }

    /**
     * 绑定表格事件
     */
    bindTableEvents() {
        // 排序事件
        document.querySelectorAll('.sortable').forEach(header => {
            header.addEventListener('click', (e) => {
                const field = e.currentTarget.dataset.field;
                this.handleSort(field);
            });
        });

        // 全选事件
        const selectAllCheckbox = document.getElementById('select-all-doctors');
        if (selectAllCheckbox) {
            selectAllCheckbox.addEventListener('change', (e) => {
                this.handleSelectAll(e.target.checked);
            });
        }

        // 单选事件
        document.querySelectorAll('.doctor-checkbox').forEach(checkbox => {
            checkbox.addEventListener('change', () => {
                this.updateSelectAllState();
            });
        });
    }

    /**
     * 渲染分页
     */
    renderPagination() {
        if (this.totalPages <= 1) return '';

        const maxVisiblePages = 5;
        let startPage = Math.max(1, this.currentPage - Math.floor(maxVisiblePages / 2));
        let endPage = Math.min(this.totalPages, startPage + maxVisiblePages - 1);

        if (endPage - startPage + 1 < maxVisiblePages) {
            startPage = Math.max(1, endPage - maxVisiblePages + 1);
        }

        let paginationHtml = `
            <div class="pagination">
                <button class="btn btn-sm btn-outline" ${this.currentPage === 1 ? 'disabled' : ''} 
                        onclick="doctorsModule.goToPage(${this.currentPage - 1})">
                    上一页
                </button>
        `;

        for (let i = startPage; i <= endPage; i++) {
            paginationHtml += `
                <button class="btn btn-sm ${i === this.currentPage ? 'btn-primary' : 'btn-outline'}" 
                        onclick="doctorsModule.goToPage(${i})">
                    ${i}
                </button>
            `;
        }

        paginationHtml += `
                <button class="btn btn-sm btn-outline" ${this.currentPage === this.totalPages ? 'disabled' : ''} 
                        onclick="doctorsModule.goToPage(${this.currentPage + 1})">
                    下一页
                </button>
                <span class="pagination-info">
                    第 ${this.currentPage} 页，共 ${this.totalPages} 页
                </span>
            </div>
        `;

        return paginationHtml;
    }

    /**
     * 处理搜索
     */
    async handleSearch(query) {
        this.currentFilter.search = query;
        this.currentPage = 1;
        await this.loadDoctors();
        this.renderDoctorsList();
    }

    /**
     * 处理筛选
     */
    async handleFilter(field, value) {
        if (value) {
            this.currentFilter[field] = value;
        } else {
            delete this.currentFilter[field];
        }
        this.currentPage = 1;
        await this.loadDoctors();
        this.renderDoctorsList();
    }

    /**
     * 处理排序
     */
    async handleSort(field) {
        if (this.currentSort.field === field) {
            this.currentSort.order = this.currentSort.order === 'asc' ? 'desc' : 'asc';
        } else {
            this.currentSort.field = field;
            this.currentSort.order = 'asc';
        }
        await this.loadDoctors();
        this.renderDoctorsList();
    }

    /**
     * 跳转到指定页面
     */
    async goToPage(page) {
        if (page < 1 || page > this.totalPages) return;
        this.currentPage = page;
        await this.loadDoctors();
        this.renderDoctorsList();
    }

    /**
     * 处理全选
     */
    handleSelectAll(checked) {
        document.querySelectorAll('.doctor-checkbox').forEach(checkbox => {
            checkbox.checked = checked;
        });
        this.updateBatchActions();
    }

    /**
     * 更新全选状态
     */
    updateSelectAllState() {
        const checkboxes = document.querySelectorAll('.doctor-checkbox');
        const selectAllCheckbox = document.getElementById('select-all-doctors');
        
        if (checkboxes.length === 0) return;
        
        const checkedCount = Array.from(checkboxes).filter(cb => cb.checked).length;
        
        if (selectAllCheckbox) {
            selectAllCheckbox.checked = checkedCount === checkboxes.length;
            selectAllCheckbox.indeterminate = checkedCount > 0 && checkedCount < checkboxes.length;
        }
        
        this.updateBatchActions();
    }

    /**
     * 更新批量操作按钮状态
     */
    updateBatchActions() {
        const selectedCount = document.querySelectorAll('.doctor-checkbox:checked').length;
        const deleteSelectedBtn = document.getElementById('delete-selected-btn');
        
        if (deleteSelectedBtn) {
            deleteSelectedBtn.disabled = selectedCount === 0;
            deleteSelectedBtn.textContent = selectedCount > 0 ? `删除选中 (${selectedCount})` : '删除选中';
        }
    }

    /**
     * 查看医生详情
     */
    async viewDoctor(doctorId) {
        try {
            const doctor = await this.app.api.getDoctor(doctorId);
            this.showDoctorModal(doctor, 'view');
        } catch (error) {
            console.error('获取医生详情失败:', error);
            this.app.showNotification('获取医生详情失败', 'error');
        }
    }

    /**
     * 编辑医生
     */
    async editDoctor(doctorId) {
        try {
            const doctor = await this.app.api.getDoctor(doctorId);
            this.showDoctorModal(doctor, 'edit');
        } catch (error) {
            console.error('获取医生信息失败:', error);
            this.app.showNotification('获取医生信息失败', 'error');
        }
    }

    /**
     * 删除医生
     */
    async deleteDoctor(doctorId) {
        if (!confirm('确定要删除这位医生吗？此操作不可恢复。')) {
            return;
        }

        try {
            await this.app.api.deleteDoctor(doctorId);
            this.app.showNotification('删除成功', 'success');
            await this.loadDoctors();
            this.renderDoctorsList();
        } catch (error) {
            console.error('删除医生失败:', error);
            this.app.showNotification('删除失败', 'error');
        }
    }

    /**
     * 批量删除
     */
    async handleBatchDelete() {
        const selectedIds = Array.from(document.querySelectorAll('.doctor-checkbox:checked'))
            .map(cb => cb.value);

        if (selectedIds.length === 0) return;

        if (!confirm(`确定要删除选中的 ${selectedIds.length} 位医生吗？此操作不可恢复。`)) {
            return;
        }

        try {
            await Promise.all(selectedIds.map(id => this.app.api.deleteDoctor(id)));
            this.app.showNotification('批量删除成功', 'success');
            await this.loadDoctors();
            this.renderDoctorsList();
        } catch (error) {
            console.error('批量删除失败:', error);
            this.app.showNotification('批量删除失败', 'error');
        }
    }

    /**
     * 显示医生模态框
     */
    showDoctorModal(doctor = null, mode = 'add') {
        const isView = mode === 'view';
        const isEdit = mode === 'edit';
        const title = isView ? '查看医生' : isEdit ? '编辑医生' : '添加医生';

        const modalHtml = `
            <div class="modal-overlay" id="doctor-modal">
                <div class="modal">
                    <div class="modal-header">
                        <h3>${title}</h3>
                        <button class="modal-close" onclick="doctorsModule.closeDoctorModal()">×</button>
                    </div>
                    <div class="modal-body">
                        <form id="doctor-form" class="form">
                            <div class="form-row">
                                <div class="form-group">
                                    <label for="doctor-name">姓名 *</label>
                                    <input type="text" id="doctor-name" class="form-input" 
                                           value="${doctor?.name || ''}" ${isView ? 'readonly' : ''} required>
                                </div>
                                <div class="form-group">
                                    <label for="doctor-title">职称 *</label>
                                    <select id="doctor-title" class="form-select" ${isView ? 'disabled' : ''} required>
                                        <option value="">请选择职称</option>
                                        <option value="主任医师" ${doctor?.title === '主任医师' ? 'selected' : ''}>主任医师</option>
                                        <option value="副主任医师" ${doctor?.title === '副主任医师' ? 'selected' : ''}>副主任医师</option>
                                        <option value="主治医师" ${doctor?.title === '主治医师' ? 'selected' : ''}>主治医师</option>
                                        <option value="住院医师" ${doctor?.title === '住院医师' ? 'selected' : ''}>住院医师</option>
                                    </select>
                                </div>
                            </div>
                            <div class="form-row">
                                <div class="form-group">
                                    <label for="doctor-hospital">医院 *</label>
                                    <input type="text" id="doctor-hospital" class="form-input" 
                                           value="${doctor?.hospital || ''}" ${isView ? 'readonly' : ''} required>
                                </div>
                                <div class="form-group">
                                    <label for="doctor-department">科室 *</label>
                                    <input type="text" id="doctor-department" class="form-input" 
                                           value="${doctor?.department || ''}" ${isView ? 'readonly' : ''} required>
                                </div>
                            </div>
                            <div class="form-row">
                                <div class="form-group">
                                    <label for="doctor-followers">粉丝数</label>
                                    <input type="number" id="doctor-followers" class="form-input" 
                                           value="${doctor?.followers_count || ''}" ${isView ? 'readonly' : ''}>
                                </div>
                                <div class="form-group">
                                    <label for="doctor-articles">文章数</label>
                                    <input type="number" id="doctor-articles" class="form-input" 
                                           value="${doctor?.articles_count || ''}" ${isView ? 'readonly' : ''}>
                                </div>
                            </div>
                            <div class="form-row">
                                <div class="form-group">
                                    <label for="doctor-avg-views">平均阅读量</label>
                                    <input type="number" id="doctor-avg-views" class="form-input" 
                                           value="${doctor?.avg_views || ''}" ${isView ? 'readonly' : ''}>
                                </div>
                                <div class="form-group">
                                    <label for="doctor-avg-likes">平均点赞数</label>
                                    <input type="number" id="doctor-avg-likes" class="form-input" 
                                           value="${doctor?.avg_likes || ''}" ${isView ? 'readonly' : ''}>
                                </div>
                            </div>
                            <div class="form-row">
                                <div class="form-group">
                                    <label for="doctor-monthly-articles">月发文量</label>
                                    <input type="number" id="doctor-monthly-articles" class="form-input" 
                                           value="${doctor?.monthly_articles || ''}" ${isView ? 'readonly' : ''}>
                                </div>
                                <div class="form-group">
                                    <label for="doctor-response-rate">回复率 (%)</label>
                                    <input type="number" id="doctor-response-rate" class="form-input" 
                                           min="0" max="100" step="0.1"
                                           value="${doctor?.response_rate || ''}" ${isView ? 'readonly' : ''}>
                                </div>
                            </div>
                            ${doctor ? `
                                <div class="form-row">
                                    <div class="form-group">
                                        <label>综合评分</label>
                                        <div class="score-display readonly">
                                            <div class="total-score">${this.app.formatNumber(doctor.total_score)}</div>
                                            <div class="score-breakdown">
                                                影响力: ${this.app.formatNumber(doctor.influence_score)} | 
                                                质量: ${this.app.formatNumber(doctor.quality_score)} | 
                                                活跃度: ${this.app.formatNumber(doctor.activity_score)}
                                            </div>
                                        </div>
                                    </div>
                                    <div class="form-group">
                                        <label>投放建议</label>
                                        <div class="investment-badge ${this.getInvestmentClass(doctor.recommendation)}">
                                            ${doctor.recommendation || '待评估'}
                                        </div>
                                    </div>
                                </div>
                            ` : ''}
                        </form>
                    </div>
                    <div class="modal-footer">
                        ${!isView ? `
                            <button type="button" class="btn btn-outline" onclick="doctorsModule.closeDoctorModal()">
                                取消
                            </button>
                            <button type="button" class="btn btn-primary" onclick="doctorsModule.saveDoctorModal('${mode}', ${doctor?.id || 'null'})">
                                ${isEdit ? '保存' : '添加'}
                            </button>
                        ` : `
                            <button type="button" class="btn btn-primary" onclick="doctorsModule.closeDoctorModal()">
                                关闭
                            </button>
                        `}
                    </div>
                </div>
            </div>
        `;

        document.body.insertAdjacentHTML('beforeend', modalHtml);
    }

    /**
     * 保存医生模态框
     */
    async saveDoctorModal(mode, doctorId) {
        const form = document.getElementById('doctor-form');
        const formData = new FormData(form);

        const doctorData = {
            name: document.getElementById('doctor-name').value.trim(),
            title: document.getElementById('doctor-title').value,
            hospital: document.getElementById('doctor-hospital').value.trim(),
            department: document.getElementById('doctor-department').value.trim(),
            followers_count: parseInt(document.getElementById('doctor-followers').value) || 0,
            articles_count: parseInt(document.getElementById('doctor-articles').value) || 0,
            avg_views: parseInt(document.getElementById('doctor-avg-views').value) || 0,
            avg_likes: parseInt(document.getElementById('doctor-avg-likes').value) || 0,
            monthly_articles: parseInt(document.getElementById('doctor-monthly-articles').value) || 0,
            response_rate: parseFloat(document.getElementById('doctor-response-rate').value) || 0
        };

        // 验证必填字段
        if (!doctorData.name || !doctorData.title || !doctorData.hospital || !doctorData.department) {
            this.app.showNotification('请填写所有必填字段', 'error');
            return;
        }

        try {
            if (mode === 'edit') {
                await this.app.api.updateDoctor(doctorId, doctorData);
                this.app.showNotification('医生信息更新成功', 'success');
            } else {
                await this.app.api.createDoctor(doctorData);
                this.app.showNotification('医生添加成功', 'success');
            }

            this.closeDoctorModal();
            await this.loadDoctors();
            this.renderDoctorsList();

        } catch (error) {
            console.error('保存医生信息失败:', error);
            this.app.showNotification('保存失败', 'error');
        }
    }

    /**
     * 关闭医生模态框
     */
    closeDoctorModal() {
        const modal = document.getElementById('doctor-modal');
        if (modal) {
            modal.remove();
        }
    }
}

// 导出模块
if (typeof module !== 'undefined' && module.exports) {
    module.exports = DoctorsModule;
} else {
    window.DoctorsModule = DoctorsModule;
}
