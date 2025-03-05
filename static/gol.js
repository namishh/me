document.addEventListener('DOMContentLoaded', function() {
    if (window.innerWidth > 1024) {
        const canvas = document.createElement('canvas');
        canvas.id = 'game-of-life-canvas';
        canvas.style.position = 'absolute';
        canvas.style.top = '20px';
        canvas.style.right = '20px';
        canvas.style.zIndex = '100';
        document.body.appendChild(canvas);

        const ctx = canvas.getContext('2d');
        const gridWidth = 25; 
        const gridHeight = 15;
        const cellSize = 10; 
        canvas.width = gridWidth * cellSize;
        canvas.height = gridHeight * cellSize; 

function initializeGrid() {
            return Array.from({ length: gridHeight }, () =>
                Array.from({ length: gridWidth }, () => Math.random() < 0.1 ? 1 : 0)
            );
        }

        let grid = initializeGrid();

        function getNextState(grid) {
            const newGrid = Array.from({ length: gridHeight }, () => Array(gridWidth).fill(0));
            for (let i = 0; i < gridHeight; i++) {
                for (let j = 0; j < gridWidth; j++) {
                    const neighbors = [
                        grid[(i - 1 + gridHeight) % gridHeight][(j - 1 + gridWidth) % gridWidth],
                        grid[(i - 1 + gridHeight) % gridHeight][j],
                        grid[(i - 1 + gridHeight) % gridHeight][(j + 1) % gridWidth],
                        grid[i][(j - 1 + gridWidth) % gridWidth],
                        grid[i][(j + 1) % gridWidth],
                        grid[(i + 1) % gridHeight][(j - 1 + gridWidth) % gridWidth],
                        grid[(i + 1) % gridHeight][j],
                        grid[(i + 1) % gridHeight][(j + 1) % gridWidth]
                    ];
                    const liveNeighbors = neighbors.reduce((acc, val) => acc + val, 0);
                    if (grid[i][j] === 1) {
                        if (liveNeighbors === 2 || liveNeighbors === 3) {
                            newGrid[i][j] = 1;
                        }
                    } else {
                        if (liveNeighbors === 3) {
                            newGrid[i][j] = 1;
                        }
                    }
                }
            }
            return newGrid;
        }

        function areGridsEqual(gridA, gridB) {
            for (let i = 0; i < gridHeight; i++) {
                for (let j = 0; j < gridWidth; j++) {
                    if (gridA[i][j] !== gridB[i][j]) {
                        return false;
                    }
                }
            }
            return true;
        }

        function isGridDead(grid) {
            return grid.every(row => row.every(cell => cell === 0));
        }

        function drawGrid() {
            const theme = document.documentElement.getAttribute('data-theme');
            const deadColor = theme === 'dark' ? '#000000' : '#f0f0f0';
            const liveColor = theme === 'dark' ? '#cccccc' : '#222222';
            const gridLineColor = theme === 'dark' ? '#222222' : '#e0e0e0';
            const borderColor = theme === 'dark' ? '#444444' : '#d0d0d0';

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
            for (let x = 0; x <= gridWidth; x++) {
                ctx.moveTo(x * cellSize, 0);
                ctx.lineTo(x * cellSize, canvas.height);
            }
            for (let y = 0; y <= gridHeight; y++) {
                ctx.moveTo(0, y * cellSize);
                ctx.lineTo(canvas.width, y * cellSize);
            }
            ctx.stroke();

            ctx.strokeStyle = borderColor;
            ctx.lineWidth = 1;
            ctx.strokeRect(0, 0, canvas.width, canvas.height);
        }

        function update() {
            const nextGrid = getNextState(grid);
            if (areGridsEqual(grid, nextGrid) || isGridDead(nextGrid) || areGridsEqual(grid, getNextState(nextGrid))) {
                grid = initializeGrid(); 
            } else {
                grid = nextGrid;
            }
            drawGrid();
        }

        drawGrid();
        setInterval(update, 600); 

        window.addEventListener('resize', function() {
            if (window.innerWidth <= 1024) {
                canvas.style.display = 'none';
            } else {
                canvas.style.display = 'block';
            }
        });
    }
});