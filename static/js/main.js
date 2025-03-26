document.addEventListener('DOMContentLoaded', () => {
    const scriptSelect = document.getElementById('scriptSelect');
    const runButton = document.getElementById('runScript');
    const output = document.getElementById('output');

    // 加载可用脚本列表
    async function loadScripts() {
        try {
            const response = await fetch('/api/scripts/list');
            const scripts = await response.json();
            
            scriptSelect.innerHTML = '<option value="">选择脚本...</option>';
            scripts.forEach(script => {
                const option = document.createElement('option');
                option.value = script.name;
                option.textContent = script.name;
                scriptSelect.appendChild(option);
            });
        } catch (error) {
            console.error('加载脚本列表失败:', error);
            output.textContent = `加载脚本列表失败: ${error.message}`;
        }
    }

    // 运行脚本
    async function runScript() {
        const scriptName = scriptSelect.value;
        if (!scriptName) {
            alert('请选择一个脚本');
            return;
        }

        output.textContent = '正在执行脚本...';
        runButton.disabled = true;

        try {
            const response = await fetch('/api/scripts/run', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    script_name: scriptName,
                    parameters: null
                }),
            });

            const result = await response.json();
            if (result.success) {
                output.textContent = result.output || '脚本执行成功，但没有输出。';
            } else {
                output.textContent = `执行失败: ${result.error}`;
            }
        } catch (error) {
            output.textContent = `请求错误: ${error.message}`;
        } finally {
            runButton.disabled = false;
        }
    }

    // 绑定事件
    runButton.addEventListener('click', runScript);

    // 初始加载脚本列表
    loadScripts();
}); 