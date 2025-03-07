document.addEventListener('DOMContentLoaded', function () {
    document.addEventListener('click', function (event) {
        const folderLabel = event.target.closest('.folder-label');
        
        if (folderLabel) {
            const directory = folderLabel.closest('.directory');
            if (directory) {
                const folderContents = directory.querySelector('.folder-contents');
                const toggleIcon = directory.querySelector('.toggle-icon');
                
                if (folderContents.classList.contains('hidden')) {
                    folderContents.classList.remove('hidden');
                    folderContents.classList.add('block');
                    toggleIcon.classList.add('rotate-90');
                } else {
                    folderContents.classList.remove('block');
                    folderContents.classList.add('hidden');
                    toggleIcon.classList.remove('rotate-90');
                }
            }
        }
    });

    function setupThemeToggle(toggleElementId) {
        const themeToggleBtn = document.getElementById(toggleElementId);
        if (themeToggleBtn) {
            const htmlElement = document.documentElement;
            
            const savedTheme = localStorage.getItem('theme') || 'dark';
            htmlElement.setAttribute('data-theme', savedTheme);
            updateThemeUI(savedTheme, themeToggleBtn);
            
            themeToggleBtn.addEventListener('click', function () {
                const currentTheme = htmlElement.getAttribute('data-theme');
                const newTheme = currentTheme === 'light' ? 'dark' : 'light';
                
                htmlElement.setAttribute('data-theme', newTheme);
                localStorage.setItem('theme', newTheme);
                
                updateThemeUI(newTheme, themeToggleBtn);
            });
        }
    }
    
    function updateThemeUI(theme, toggleBtn) {
        if (theme === 'dark') {
            toggleBtn.classList.remove('ph-moon-stars');
            toggleBtn.classList.add('ph-sun');
        } else {
            toggleBtn.classList.remove('ph-sun');
            toggleBtn.classList.add('ph-moon-stars');
        }
    }
    
    setupThemeToggle('theme-toggle');
    setupThemeToggle('mobile-theme-toggle');
    
    const mobileMenuToggle = document.getElementById('mobile-menu-toggle');
    const sidebar = document.getElementById('sidebar');
    const sidebarOverlay = document.getElementById('sidebar-overlay');
    
    if (mobileMenuToggle && sidebar && sidebarOverlay) {
        mobileMenuToggle.addEventListener('click', function () {
            sidebar.classList.toggle('open');
            sidebarOverlay.classList.toggle('open');
        });
        
        sidebarOverlay.addEventListener('click', function () {
            sidebar.classList.remove('open');
            sidebarOverlay.classList.remove('open');
        });
    }
    
    function expandToCurrentFile() {
        const currentPath = window.location.pathname;
        const fileLinks = document.querySelectorAll('.file-link');
        
        const activeFileLink = Array.from(fileLinks).find(link => {
            const href = link.getAttribute('href');
            return href === currentPath || (href.endsWith('/') && currentPath === href.slice(0, -1));
        });
        
        if (activeFileLink) {
            fileLinks.forEach(link => {
                link.classList.remove('font-bold');
            });
            
            activeFileLink.classList.add('font-bold');
            
            let parentDirectory = activeFileLink.closest('.directory');
            while (parentDirectory) {
                const folderContents = parentDirectory.querySelector('.folder-contents');
                const toggleIcon = parentDirectory.querySelector('.toggle-icon').querySelector('.ph-caret-right');
                
                if (folderContents) {
                    folderContents.classList.remove('hidden');
                    folderContents.classList.add('block');
                }
                
                if (toggleIcon) {
                    toggleIcon.classList.add('rotate-90');
                    toggleIcon.classList.remove('rotate-0'); 
                }
                
                parentDirectory = parentDirectory.parentElement.closest('.directory');
            }
        }
    }
    
    expandToCurrentFile();
});