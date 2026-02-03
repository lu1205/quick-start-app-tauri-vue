<script setup>
import { ref, reactive, computed } from "vue";
import { open } from "@tauri-apps/plugin-dialog";

// 软件列表数据
const softwareList = ref([]);

// 搜索关键词
const searchKeyword = ref("");

// 弹窗状态
const showAddModal = ref(false);
const showContextMenu = ref(false);
const contextMenuPosition = ref({ x: 0, y: 0 });
const selectedSoftware = ref(null);

// 新软件表单数据
const newSoftware = reactive({
  name: "",
  path: "",
  icon: "https://trae-api-cn.mchost.guru/api/ide/v1/text_to_image?prompt=laptop%20icon%20blue%20outline%20on%20white%20background&image_size=square"
});

// 过滤后的软件列表
const filteredSoftwareList = computed(() => {
  if (!searchKeyword.value) {
    return softwareList.value;
  }
  return softwareList.value.filter(software => 
    software.name.toLowerCase().includes(searchKeyword.value.toLowerCase())
  );
});

// 打开添加软件弹窗
function openAddModal() {
  // 重置表单
  newSoftware.name = "";
  newSoftware.path = "";
  newSoftware.icon = "https://trae-api-cn.mchost.guru/api/ide/v1/text_to_image?prompt=laptop%20icon%20blue%20outline%20on%20white%20background&image_size=square";
  showAddModal.value = true;
}

// 关闭添加软件弹窗
function closeAddModal() {
  showAddModal.value = false;
}

// 保存新软件
function saveSoftware() {
  if (newSoftware.name && newSoftware.path) {
    const newId = softwareList.value.length > 0 
      ? Math.max(...softwareList.value.map(s => s.id)) + 1 
      : 1;
    
    softwareList.value.push({
      id: newId,
      name: newSoftware.name,
      path: newSoftware.path,
      icon: newSoftware.icon
    });
    
    closeAddModal();
  }
}

// 打开右键菜单
function openContextMenu(event, software) {
  event.preventDefault();
  contextMenuPosition.value = {
    x: event.clientX,
    y: event.clientY
  };
  selectedSoftware.value = software;
  showContextMenu.value = true;
}

// 关闭右键菜单
function closeContextMenu() {
  showContextMenu.value = false;
  selectedSoftware.value = null;
}

// 编辑软件
function editSoftware() {
  if (selectedSoftware.value) {
    newSoftware.name = selectedSoftware.value.name;
    newSoftware.path = selectedSoftware.value.path;
    newSoftware.icon = selectedSoftware.value.icon;
    showAddModal.value = true;
    closeContextMenu();
  }
}

// 删除软件
function deleteSoftware() {
  if (selectedSoftware.value) {
    softwareList.value = softwareList.value.filter(
      software => software.id !== selectedSoftware.value.id
    );
    closeContextMenu();
  }
}

// 选择文件路径
async function selectFilePath() {
  try {
    // 尝试使用@tauri-apps/plugin-dialog插件的open函数打开文件选择对话框
    const selected = await open({
      multiple: false,
      directory: false,
      filters: [
        {
          name: "应用程序",
          extensions: ["exe", "lnk", "*"]
        }
      ]
    });
    console.log('selected',selected);
    
    
    if (selected) {
      newSoftware.path = selected;
      
      // 从路径中提取应用名称
      const pathParts = selected.split("\\");
      const fileName = pathParts[pathParts.length - 1];
      newSoftware.name = fileName.replace(/\.(exe|lnk)$/i, "");
      
      // 调用get_file_icon命令获取文件图标
      try {
        // 使用window.__TAURI_INTERNALS__.invoke来调用Tauri命令
        const iconPath = await window.__TAURI_INTERNALS__.invoke("get_file_icon", { path: selected });
        newSoftware.icon = iconPath;
      } catch (iconError) {
        console.error("Error getting file icon:", iconError);
        // 出错时使用在线API生成图标作为备选
        newSoftware.icon = `https://trae-api-cn.mchost.guru/api/ide/v1/text_to_image?prompt=${encodeURIComponent(newSoftware.name)}%20software%20icon%20colorful%20modern%20design&image_size=square`;
      }
    }
  } catch (error) {
    console.error("Error opening file dialog:", error);
    // 出错时使用手动输入路径的方式作为备选
    const path = prompt("请输入应用程序路径（.exe或.lnk文件）：");
    if (path) {
      newSoftware.path = path;
      
      // 从路径中提取应用名称
      const pathParts = path.split("\\");
      const fileName = pathParts[pathParts.length - 1];
      newSoftware.name = fileName.replace(/\.(exe|lnk)$/i, "");
      
      // 调用get_file_icon命令获取文件图标
      try {
        // 使用window.__TAURI_INTERNALS__.invoke来调用Tauri命令
        const iconPath = await window.__TAURI_INTERNALS__.invoke("get_file_icon", { path: path });
        newSoftware.icon = iconPath;
      } catch (iconError) {
        console.error("Error getting file icon:", iconError);
        // 出错时使用在线API生成图标作为备选
        newSoftware.icon = `https://trae-api-cn.mchost.guru/api/ide/v1/text_to_image?prompt=${encodeURIComponent(newSoftware.name)}%20software%20icon%20colorful%20modern%20design&image_size=square`;
      }
    }
  }
}

// 选择图标
function selectIcon() {
  // 这里可以集成Tauri的文件选择对话框
  // 暂时使用默认图标
  newSoftware.icon = "https://trae-api-cn.mchost.guru/api/ide/v1/text_to_image?prompt=random%20software%20icon%20colorful%20modern%20design&image_size=square";
}

// 搜索软件
function searchSoftware() {
  // 搜索功能已通过computed实现
}

// 打开软件
async function openSoftware(software) {
  try {
    // 使用window.__TAURI_INTERNALS__.invoke来调用Tauri命令
    await window.__TAURI_INTERNALS__.invoke("open_software", { path: software.path });
  } catch (error) {
    console.error("Error opening software:", error);
    alert("无法打开软件，请检查路径是否正确");
  }
}
</script>

<template>
  <div class="app">
    <!-- 顶部导航栏 -->
    <header class="header">
      <h1 class="app-title">软件管理器</h1>
      <div class="search-bar">
        <input 
          type="text" 
          v-model="searchKeyword" 
          placeholder="搜索软件..."
          class="search-input"
        />
        <button class="search-btn" @click="searchSoftware">搜索</button>
        <button class="add-btn" @click="openAddModal">添加软件</button>
      </div>
    </header>

    <!-- 软件列表 -->
    <main class="software-list">
      <div 
        v-for="software in filteredSoftwareList" 
        :key="software.id"
        class="software-card"
        @click="openSoftware(software)"
        @contextmenu="openContextMenu($event, software)"
      >
        <img :src="software.icon" alt="软件图标" class="software-icon" />
        <p class="software-name">{{ software.name }}</p>
      </div>
    </main>

    <!-- 添加/编辑软件弹窗 -->
    <div v-if="showAddModal" class="modal-overlay" @click="closeAddModal">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h2>添加软件</h2>
          <button class="close-btn" @click="closeAddModal">&times;</button>
        </div>
        <div class="modal-body">
          <div class="form-group">
            <label>软件名称</label>
            <input 
              type="text" 
              v-model="newSoftware.name" 
              placeholder="请输入软件名称"
              class="form-input"
            />
          </div>
          <div class="form-group">
            <label>软件路径</label>
            <div class="path-input-group">
              <input 
                type="text" 
                v-model="newSoftware.path" 
                placeholder="请输入软件路径或点击"
                class="form-input path-input"
              />
              <button class="select-btn" @click="selectFilePath">选择文件</button>
            </div>
          </div>
          <div class="form-group">
            <label>软件图标</label>
            <div class="icon-select-group">
              <img :src="newSoftware.icon" alt="软件图标" class="icon-preview" />
              <button class="select-btn" @click="selectIcon">选择图标</button>
            </div>
          </div>
        </div>
        <div class="modal-footer">
          <button class="save-btn" @click="saveSoftware">保存</button>
          <button class="cancel-btn" @click="closeAddModal">取消</button>
        </div>
      </div>
    </div>

    <!-- 右键菜单 -->
    <div 
      v-if="showContextMenu" 
      class="context-menu"
      :style="{
        left: contextMenuPosition.x + 'px',
        top: contextMenuPosition.y + 'px'
      }"
      @click.stop
    >
      <div class="context-menu-item" @click="editSoftware">编辑</div>
      <div class="context-menu-item" @click="deleteSoftware">删除</div>
    </div>
  </div>
</template>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

.app {
  min-height: 100vh;
  padding: 20px;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 30px;
  padding-bottom: 15px;
  border-bottom: 1px solid #e0e0e0;
}

.app-title {
  font-size: 24px;
  font-weight: 600;
  color: #333;
}

.search-bar {
  display: flex;
  align-items: center;
  gap: 10px;
}

.search-input {
  width: 300px;
  padding: 8px 12px;
  border: 1px solid #d0d0d0;
  border-radius: 4px;
  font-size: 14px;
}

.search-btn {
  padding: 8px 16px;
  border: 1px solid #d0d0d0;
  border-radius: 4px;
  background-color: #f5f5f5;
  cursor: pointer;
  font-size: 14px;
}

.add-btn {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  background-color: #396cd8;
  color: white;
  cursor: pointer;
  font-size: 14px;
}

.software-list {
  display: flex;
  flex-wrap: wrap;
  gap: 20px;
}

.software-card {
  width: 120px;
  height: 140px;
  background-color: white;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  padding: 15px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: transform 0.2s ease, box-shadow 0.2s ease;
}

.software-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.software-icon {
  width: 60px;
  height: 60px;
  border-radius: 8px;
  margin-bottom: 10px;
}

.software-name {
  font-size: 14px;
  color: #333;
  text-align: center;
  word-break: break-all;
}

/* 弹窗样式 */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background-color: white;
  border-radius: 8px;
  width: 500px;
  max-width: 90%;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.2);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 15px 20px;
  border-bottom: 1px solid #e0e0e0;
}

.modal-header h2 {
  font-size: 18px;
  font-weight: 600;
  color: #333;
}

.close-btn {
  background: none;
  border: none;
  font-size: 24px;
  cursor: pointer;
  color: #999;
  padding: 0;
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.modal-body {
  padding: 20px;
}

.form-group {
  margin-bottom: 20px;
}

.form-group label {
  display: block;
  margin-bottom: 8px;
  font-size: 14px;
  color: #666;
}

.form-input {
  width: 100%;
  padding: 8px 12px;
  border: 1px solid #d0d0d0;
  border-radius: 4px;
  font-size: 14px;
}

.path-input-group {
  display: flex;
  gap: 10px;
}

.path-input {
  flex: 1;
}

.icon-select-group {
  display: flex;
  align-items: center;
  gap: 15px;
}

.icon-preview {
  width: 60px;
  height: 60px;
  border-radius: 8px;
  border: 1px solid #e0e0e0;
}

.select-btn {
  padding: 8px 16px;
  border: 1px solid #d0d0d0;
  border-radius: 4px;
  background-color: #f5f5f5;
  cursor: pointer;
  font-size: 14px;
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  padding: 15px 20px;
  border-top: 1px solid #e0e0e0;
}

.save-btn {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  background-color: #396cd8;
  color: white;
  cursor: pointer;
  font-size: 14px;
}

.cancel-btn {
  padding: 8px 16px;
  border: 1px solid #d0d0d0;
  border-radius: 4px;
  background-color: #f5f5f5;
  cursor: pointer;
  font-size: 14px;
}

/* 右键菜单样式 */
.context-menu {
  position: fixed;
  background-color: white;
  border-radius: 4px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
  z-index: 1001;
  min-width: 100px;
}

.context-menu-item {
  padding: 8px 16px;
  font-size: 14px;
  cursor: pointer;
  transition: background-color 0.2s ease;
}

.context-menu-item:hover {
  background-color: #f5f5f5;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .header {
    flex-direction: column;
    align-items: flex-start;
    gap: 15px;
  }
  
  .search-bar {
    width: 100%;
  }
  
  .search-input {
    flex: 1;
  }
  
  .software-list {
    justify-content: center;
  }
}
</style>
