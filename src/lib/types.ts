export interface Annotation {
    id: string;
    type: AnnotationType;
    x: number;
    y: number;
    width?: number;
    height?: number;
    color: string;
    strokeWidth: number;
    data?: any; // For text content, points array, etc.
}

export type AnnotationType =
    | 'pen'
    | 'highlighter'
    | 'highlight'
    | 'speechBubble'
    | 'rectangle'
    | 'ellipse'
    | 'line'
    | 'arrow'
    | 'text'
    | 'step'
    | 'pixelate'
    | 'crop';

export interface TabData {
    id: string;
    label: string;
    imagePath: string;
    originalImagePath: string;
    annotations: Annotation[];
    history: Annotation[][]; // Stack of annotation states
    historyIndex: number;    // Current position in history
    isDirty: boolean;
    timestamp: number;
    stepCounter: number;
}

export type Tool = AnnotationType | 'select';

export interface AppConfig {
    screenshotDirectory: string;
    copyAfterCapture: boolean;
    defaultColor: string;
    defaultStrokeWidth: 'Thin' | 'Medium' | 'Thick';
    lastSaveDirectory?: string;
}
