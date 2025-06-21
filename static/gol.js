document.addEventListener('DOMContentLoaded', function() {
    if (window.innerWidth <= 1024) return;

    const canvas = document.createElement('canvas');
    canvas.id = 'game-of-life-canvas';
    Object.assign(canvas.style, {
        position: 'absolute',
        top: '20px',
        right: '20px',
        zIndex: '100',
        opacity: '0.75',
    });
    document.body.appendChild(canvas);

    const ctx = canvas.getContext('2d');
    const gridWidth = 25;
    const gridHeight = 15;
    const cellSize = 10;
    canvas.width = gridWidth * cellSize;
    canvas.height = gridHeight * cellSize;

    const neighborOffsets = [
        [-1, -1], [-1, 0], [-1, 1],
        [0, -1],          [0, 1],
        [1, -1],  [1, 0],  [1, 1]
    ];

    let deadColor, liveColor, gridLineColor, borderColor;

    function updateThemeColors() {
        const theme = document.documentElement.getAttribute('data-theme');
        deadColor = theme === 'dark' ? '#000000' : '#f0f0f0';
        liveColor = theme === 'dark' ? '#cccccc' : '#222222';
        gridLineColor = theme === 'dark' ? '#222222' : '#e0e0e0';
        borderColor = theme === 'dark' ? '#444444' : '#d0d0d0';
        drawGrid(); // Immediate redraw with new colors
    }

    function initializeGrid() {
        return Array.from({ length: gridHeight }, () =>
            Array.from({ length: gridWidth }, () => Math.random() < 0.1 ? 1 : 0)
        );
    }

    let grid = initializeGrid();

    function getNextState(currentGrid) {
        const newGrid = Array.from({ length: gridHeight }, () => Array(gridWidth).fill(0));
        for (let i = 0; i < gridHeight; i++) {
            for (let j = 0; j < gridWidth; j++) {
                let liveNeighbors = 0;
                for (const [dx, dy] of neighborOffsets) {
                    const ni = (i + dx + gridHeight) % gridHeight;
                    const nj = (j + dy + gridWidth) % gridWidth;
                    liveNeighbors += currentGrid[ni][nj];
                }
                if (currentGrid[i][j] === 1) {
                    newGrid[i][j] = (liveNeighbors === 2 || liveNeighbors === 3) ? 1 : 0;
                } else {
                    newGrid[i][j] = (liveNeighbors === 3) ? 1 : 0;
                }
            }
        }
        return newGrid;
    }

    function areGridsEqual(gridA, gridB) {
        for (let i = 0; i < gridHeight; i++) {
            for (let j = 0; j < gridWidth; j++) {
                if (gridA[i][j] !== gridB[i][j]) return false;
            }
        }
        return true;
    }

    function isGridDead(currentGrid) {
        for (let i = 0; i < gridHeight; i++) {
            for (let j = 0; j < gridWidth; j++) {
                if (currentGrid[i][j] === 1) return false;
            }
        }
        return true;
    }

    function drawGrid() {
        ctx.fillStyle = deadColor;
        ctx.fillRect(0, 0, canvas.width, canvas.height);

        ctx.fillStyle = liveColor;
        for (let i = 0; i < gridHeight; i++) {
            for (let j = 0; j < gridWidth; j++) {
                if (grid[i][j] === 1) {
                    ctx.fillRect(j * cellSize, i * cellSize, cellSize, cellSize);
                }
            }
        }

        ctx.strokeStyle = gridLineColor;
        ctx.lineWidth = 1;
        ctx.beginPath();
        for (let x = 0; x <= canvas.width; x += cellSize) {
            ctx.moveTo(x, 0);
            ctx.lineTo(x, canvas.height);
        }
        for (let y = 0; y <= canvas.height; y += cellSize) {
            ctx.moveTo(0, y);
            ctx.lineTo(canvas.width, y);
        }
        ctx.stroke();

        ctx.strokeStyle = borderColor;
        ctx.strokeRect(0, 0, canvas.width, canvas.height);
    }

    function updateSimulation() {
        const nextGrid = getNextState(grid);
        if (areGridsEqual(grid, nextGrid) || 
            isGridDead(nextGrid) || 
            areGridsEqual(nextGrid, getNextState(nextGrid))) {
            grid = initializeGrid();
        } else {
            grid = nextGrid;
        }
    }

    let lastUpdateTime = performance.now();
    let animationId;

    function gameLoop(currentTime) {
        if (currentTime - lastUpdateTime >= 600) {
            updateSimulation();
            lastUpdateTime = currentTime;
        }
        drawGrid();
        animationId = requestAnimationFrame(gameLoop);
    }

    animationId = requestAnimationFrame(gameLoop);

    window.addEventListener('resize', function() {
        if (window.innerWidth <= 1024) {
            canvas.style.display = 'none';
            if (animationId) {
                cancelAnimationFrame(animationId);
                animationId = null;
            }
        } else if (canvas.style.display === 'none') {
            canvas.style.display = 'block';
            lastUpdateTime = performance.now();
            animationId = requestAnimationFrame(gameLoop);
        }
    });

    const observer = new MutationObserver(mutations => {
        mutations.forEach(mutation => {
            if (mutation.attributeName === 'data-theme') {
                updateThemeColors();
            }
        });
    });

    observer.observe(document.documentElement, {
        attributes: true,
        attributeFilter: ['data-theme']
    });

    updateThemeColors();
});