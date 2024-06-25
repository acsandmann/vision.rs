# @asandmann/vision

This module provides simple and easy-to-use bindings to Apple's vision-framework and more specifically their VNRecognizeTextRequest function. As such, this will only run on machines running Apple software.

### Example

```typescript
import { vnRecognizeTextRequest } from '@asandmann/vision';
import { readFile } from 'fs/promises';

const buffer = await readFile('path/to/image.png');

// Using the function
const res1 = vnRecognizeTextRequest( buffer);
console.log('Recognized text:', res1.join('\n'));

```