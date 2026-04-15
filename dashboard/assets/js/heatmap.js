// ═══════════════════════════════════════════════════════════════════════════════
//  SENTIENT OS - Heatmap Diff Viewer
// ═══════════════════════════════════════════════════════════════════════════════
//  Color-coded risk annotation for code review
//  - Red: High risk areas
//  - Yellow: Medium risk
//  - Green: Safe areas
// ═══════════════════════════════════════════════════════════════════════════════

class HeatmapViewer {
    constructor(containerId) {
        this.container = document.getElementById(containerId);
        this.data = null;
        this.annotations = [];
        this.riskLevels = {
            low: { color: '#4CAF50', label: 'Low Risk', score: 0 },
            medium: { color: '#FF9800', label: 'Medium Risk', score: 50 },
            high: { color: '#F44336', label: 'High Risk', score: 80 },
            critical: { color: '#9C27B0', label: 'Critical', score: 100 }
        };
    }
    
    // Load diff data
    async loadDiff(diffUrl) {
        try {
            const response = await fetch(diffUrl);
            const diffText = await response.text();
            this.parseDiff(diffText);
            this.render();
        } catch (error) {
            console.error('Failed to load diff:', error);
        }
    }
    
    // Parse unified diff
    parseDiff(diffText) {
        const lines = diffText.split('\n');
        const files = [];
        let currentFile = null;
        
        for (const line of lines) {
            if (line.startsWith('diff --git')) {
                if (currentFile) files.push(currentFile);
                currentFile = {
                    path: line.split(' ')[2].replace('a/', ''),
                    additions: 0,
                    deletions: 0,
                    hunks: [],
                    riskScore: 0,
                    annotations: []
                };
            } else if (line.startsWith('+++') || line.startsWith('---')) {
                continue;
            } else if (line.startsWith('@@')) {
                currentFile?.hunks.push({
                    header: line,
                    lines: []
                });
            } else if (currentFile?.hunks.length > 0) {
                const hunk = currentFile.hunks[currentFile.hunks.length - 1];
                if (line.startsWith('+')) {
                    currentFile.additions++;
                    hunk.lines.push({ type: 'add', content: line.substring(1), risk: 0 });
                } else if (line.startsWith('-')) {
                    currentFile.deletions++;
                    hunk.lines.push({ type: 'del', content: line.substring(1), risk: 0 });
                } else {
                    hunk.lines.push({ type: 'ctx', content: line.substring(1), risk: 0 });
                }
            }
        }
        
        if (currentFile) files.push(currentFile);
        this.data = { files };
        this.analyzeRisks();
    }
    
    // Analyze risk for each file and line
    analyzeRisks() {
        if (!this.data) return;
        
        const riskPatterns = {
            high: [
                /password/i, /secret/i, /api[_-]?key/i, /token/i,
                /eval\(/, /innerHTML/, /dangerouslySetInnerHTML/,
                /exec\(/, /system\(/, /shell\(/,
                /DELETE\s+FROM/i, /DROP\s+TABLE/i
            ],
            medium: [
                /TODO/i, /FIXME/i, /HACK/i, /XXX/i,
                /console\.log/, /debugger/,
                /catch\s*\(\s*\)/, // Empty catch
                /var\s+/, // var usage
            ],
            low: [
                /\/\//, /\/\*/, /#.*$/, // Comments
            ]
        };
        
        for (const file of this.data.files) {
            let totalRisk = 0;
            let lineCount = 0;
            
            for (const hunk of file.hunks) {
                for (const line of hunk.lines) {
                    if (line.type === 'ctx') continue;
                    
                    lineCount++;
                    let lineRisk = 0;
                    
                    // Check patterns
                    for (const pattern of riskPatterns.high) {
                        if (pattern.test(line.content)) {
                            lineRisk = 100;
                            file.annotations.push({
                                line: line.content,
                                type: 'high',
                                message: `High risk pattern: ${pattern}`
                            });
                        }
                    }
                    
                    for (const pattern of riskPatterns.medium) {
                        if (pattern.test(line.content) && lineRisk < 80) {
                            lineRisk = 50;
                        }
                    }
                    
                    line.risk = lineRisk;
                    totalRisk += lineRisk;
                }
            }
            
            file.riskScore = lineCount > 0 ? Math.round(totalRisk / lineCount) : 0;
        }
    }
    
    // Render heatmap
    render() {
        if (!this.container || !this.data) return;
        
        let html = '<div class="heatmap-container">';
        
        // Summary
        html += this.renderSummary();
        
        // Files
        html += '<div class="heatmap-files">';
        for (const file of this.data.files) {
            html += this.renderFile(file);
        }
        html += '</div>';
        
        html += '</div>';
        this.container.innerHTML = html;
        this.attachEventListeners();
    }
    
    renderSummary() {
        const totalAdditions = this.data.files.reduce((sum, f) => sum + f.additions, 0);
        const totalDeletions = this.data.files.reduce((sum, f) => sum + f.deletions, 0);
        const avgRisk = Math.round(
            this.data.files.reduce((sum, f) => sum + f.riskScore, 0) / (this.data.files.length || 1)
        );
        
        return `
            <div class="heatmap-summary">
                <div class="summary-item">
                    <span class="label">Files Changed</span>
                    <span class="value">${this.data.files.length}</span>
                </div>
                <div class="summary-item additions">
                    <span class="label">Additions</span>
                    <span class="value">+${totalAdditions}</span>
                </div>
                <div class="summary-item deletions">
                    <span class="label">Deletions</span>
                    <span class="value">-${totalDeletions}</span>
                </div>
                <div class="summary-item risk-${this.getRiskLevel(avgRisk)}">
                    <span class="label">Avg Risk</span>
                    <span class="value">${avgRisk}%</span>
                </div>
            </div>
        `;
    }
    
    renderFile(file) {
        const riskLevel = this.getRiskLevel(file.riskScore);
        const riskColor = this.riskLevels[riskLevel].color;
        
        let html = `
            <div class="heatmap-file" data-path="${file.path}">
                <div class="file-header" style="border-left: 4px solid ${riskColor}">
                    <span class="file-path">${file.path}</span>
                    <span class="file-stats">
                        <span class="additions">+${file.additions}</span>
                        <span class="deletions">-${file.deletions}</span>
                        <span class="risk-badge ${riskLevel}">${file.riskScore}%</span>
                    </span>
                </div>
                <div class="file-content" style="display: none;">
        `;
        
        for (const hunk of file.hunks) {
            html += `<div class="hunk">`;
            html += `<div class="hunk-header">${this.escapeHtml(hunk.header)}</div>`;
            html += `<div class="hunk-lines">`;
            
            for (const line of hunk.lines) {
                const lineRisk = this.getRiskLevel(line.risk);
                const bgColor = line.type === 'add' ? 'rgba(76, 175, 80, 0.1)' 
                              : line.type === 'del' ? 'rgba(244, 67, 54, 0.1)' 
                              : '';
                
                html += `
                    <div class="line line-${line.type} risk-${lineRisk}" 
                         style="background: ${bgColor}">
                        <span class="line-type">${line.type === 'add' ? '+' : line.type === 'del' ? '-' : ' '}</span>
                        <span class="line-content">${this.escapeHtml(line.content)}</span>
                        ${line.risk > 0 ? `<span class="risk-indicator" style="background: ${this.riskLevels[lineRisk].color}"></span>` : ''}
                    </div>
                `;
            }
            
            html += '</div></div>';
        }
        
        html += '</div></div>';
        return html;
    }
    
    getRiskLevel(score) {
        if (score >= 80) return 'critical';
        if (score >= 50) return 'high';
        if (score >= 20) return 'medium';
        return 'low';
    }
    
    escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }
    
    attachEventListeners() {
        // Toggle file content
        document.querySelectorAll('.file-header').forEach(header => {
            header.addEventListener('click', () => {
                const content = header.nextElementSibling;
                content.style.display = content.style.display === 'none' ? 'block' : 'none';
            });
        });
        
        // Hover for annotations
        document.querySelectorAll('.line').forEach(line => {
            line.addEventListener('mouseenter', (e) => {
                // Show tooltip with risk info
            });
        });
    }
}

// CSS styles
const heatmapStyles = `
.heatmap-container {
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
    font-size: 13px;
    background: #0F0F0F;
    color: #E0E0E0;
    border-radius: 8px;
    overflow: hidden;
}

.heatmap-summary {
    display: flex;
    gap: 20px;
    padding: 15px 20px;
    background: #1A1D21;
    border-bottom: 1px solid #2A2D31;
}

.summary-item {
    display: flex;
    flex-direction: column;
}

.summary-item .label {
    font-size: 11px;
    color: #808080;
    text-transform: uppercase;
}

.summary-item .value {
    font-size: 18px;
    font-weight: 600;
}

.additions .value { color: #4CAF50; }
.deletions .value { color: #F44336; }

.heatmap-files {
    max-height: 600px;
    overflow-y: auto;
}

.heatmap-file {
    border-bottom: 1px solid #2A2D31;
}

.file-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px 15px;
    cursor: pointer;
    transition: background 0.2s;
}

.file-header:hover {
    background: #1A1D21;
}

.file-path {
    font-weight: 500;
}

.file-stats {
    display: flex;
    gap: 15px;
    font-size: 12px;
}

.risk-badge {
    padding: 2px 8px;
    border-radius: 4px;
    font-weight: 600;
}

.risk-badge.low { background: rgba(76, 175, 80, 0.2); color: #4CAF50; }
.risk-badge.medium { background: rgba(255, 152, 0, 0.2); color: #FF9800; }
.risk-badge.high { background: rgba(244, 67, 54, 0.2); color: #F44336; }
.risk-badge.critical { background: rgba(156, 39, 176, 0.2); color: #9C27B0; }

.file-content {
    padding: 0 15px 15px;
}

.hunk-header {
    color: #808080;
    padding: 8px 0;
    font-size: 11px;
}

.hunk-lines {
    background: #0A0A0A;
    border-radius: 4px;
    overflow: hidden;
}

.line {
    display: flex;
    align-items: center;
    padding: 2px 10px;
    border-left: 3px solid transparent;
}

.line-add { border-left-color: #4CAF50; }
.line-del { border-left-color: #F44336; }

.line-type {
    width: 15px;
    text-align: center;
    color: #606060;
}

.line-content {
    flex: 1;
    white-space: pre;
    overflow-x: auto;
}

.risk-indicator {
    width: 4px;
    height: 4px;
    border-radius: 50%;
    margin-left: 10px;
}
`;

// Inject styles
const styleSheet = document.createElement('style');
styleSheet.textContent = heatmapStyles;
document.head.appendChild(styleSheet);

// Export
window.HeatmapViewer = HeatmapViewer;
