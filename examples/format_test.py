import re

with open(r'C:\Users\X1 CARBON\Downloads\repomd\ourcreativity_test.md', 'r', encoding='utf-8') as f:
    text = f.read()

# Find all files to build the repository length
files = re.findall(r'## File: (.*?) \(CPS: (.*?), Level: (.*?)\)', text)

out = '<?xml version="1.0" encoding="UTF-8"?>\n'
out += '<context>\n'
out += '  <repository_details>\n'
out += '    <description>The following is an optimized codebase extraction. Files are compressed according to context priority.</description>\n'
out += f'    <total_files_scanned>{len(files)}</total_files_scanned>\n'
out += '  </repository_details>\n\n'

out += '  <repository_structure>\n'
for file, cps, level in files:
    out += f'    - {file}\n'
out += '  </repository_structure>\n\n'

out += '  <code_base>\n'

def replacer(match):
    file, cps, level, block = match.groups()
    ext = file.split('.')[-1] if '.' in file else 'text'
    # clean block
    content = block.strip()
    if content.startswith('```'): content = content[3:].strip()
    if content.endswith('```'): content = content[:-3].strip()
    
    return f'    <file path="{file}" cps="{cps}" level="{level}">\n```{ext}\n{content}\n```\n    </file>\n\n'

new_text = re.sub(r'## File: (.*?) \(CPS: (.*?), Level: (.*?)\)\n\n(```.*?```)\n', replacer, text, flags=re.DOTALL)
new_text = re.sub(r'> \[Limit Reached.*?\n', '', new_text)

# Clean up initial cruft
if '# Repository Context' in new_text:
    new_text = new_text.split('# Repository Context', 1)[1]

out += new_text.strip() + '\n'
out += '  </code_base>\n'
out += '</context>\n'

with open(r'C:\Users\X1 CARBON\Downloads\repomd\ourcreativity_demo.md', 'w', encoding='utf-8') as f:
    f.write(out)
