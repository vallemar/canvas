import { Color, LayoutBase, Property } from '@nativescript/core';
import { Canvas } from '../Canvas';

export const paintStyleProperty = new Property<Paint, 'fill' | 'stroke'>({
	name: 'paintStyle',
	defaultValue: 'fill',
});

export const strokeWidthProperty = new Property<Paint, number>({
	name: 'strokeWidth',
	defaultValue: 1,
});

const defaultColor = new Color('black');
export class Paint extends LayoutBase {
	strokeWidth: number;
	_canvas: Canvas;
	_addCanvas(canvas: Canvas) {
		this._canvas = canvas;
	}

	paintStyle: 'fill' | 'stroke';

	_paintStyleDirty = false;

	_inGroup = false;

	[paintStyleProperty.setNative](value) {
		this._paintStyleDirty = true;
	}

	_getStrokeWidth() {
		return (this.parent as any)?.strokeWidth ?? this.strokeWidth;
	}
	_getPaintStyle() {
		const paintStyle = this.paintStyle;
		if (paintStyle !== 'fill') {
			return paintStyle;
		}
		if (this._inGroup) {
			return (this.parent as any).paintStyle;
		}
		return 'fill';
	}

	_getColor() {
		const color = this.color ?? defaultColor;
		const hex = color.hex;
		if (color.name !== 'black' && hex !== '#000000') {
			return hex;
		}
		return (this.parent as any)?.color?.hex ?? hex;
	}

	draw() {
		const color = this._getColor();

		const context = this._canvas.getContext('2d') as any as CanvasRenderingContext2D;

		const style = this._getPaintStyle();

		context.closePath();

		if (style === 'fill') {
			context.fillStyle = color;
			context.fill();
		} else if (style === 'stroke') {
			context.lineWidth = this._getStrokeWidth();
			context.strokeStyle = color;
			context.stroke();
		}

		context.beginPath();
	}
}

paintStyleProperty.register(Paint);
strokeWidthProperty.register(Paint);
