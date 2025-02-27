//
//  NSCCanvas.swift
//
//  Created by Osei Fortune on 7/14/19.
//  Copyright © 2019 Osei Fortune. All rights reserved.
//

import Foundation
import UIKit
import GLKit
import WebKit

@objcMembers
@objc(NSCCanvas)
public class NSCCanvas: UIView {
    
    
    private static let shared_context_view = GLKView(frame: .init(x: 0, y: 0, width: 1, height: 1))
    
    private static var _shared_context: Int64 = 0
    private static var shared_context: Int64 {
        get {
            
            if(_shared_context != 0){
                return _shared_context
            }
            
            var properties: [String: Any] = [:]
            
            properties[kEAGLDrawablePropertyColorFormat] = kEAGLColorFormatRGBA8
            shared_context_view.isOpaque = false
            (shared_context_view.layer as! CAEAGLLayer).isOpaque = false
            
            let eaglLayer = shared_context_view.layer as! CAEAGLLayer
            eaglLayer.drawableProperties = properties
            
            shared_context_view.drawableDepthFormat = .format24
            
    
            let ptr = Unmanaged.passRetained(shared_context_view).toOpaque()
            let viewPtr = Int64(Int(bitPattern: ptr))
            
       
            
            _shared_context = CanvasHelpers.initGLWithView(viewPtr, true, true, true, false, "default", true, false, false, false, false, 2, false)
            
            
            return _shared_context
            
        }
    }
    
    public static let store = NSMutableDictionary()
    
    private static var views: NSMapTable<NSString,NSCCanvas> = NSMapTable(keyOptions: .copyIn, valueOptions: .weakMemory)
    
    public static func getViews() -> NSMapTable<NSString,NSCCanvas> {
        return views
    }
    
    var ptr: UnsafeMutableRawPointer? = nil
    
    public func getViewPtr() -> UnsafeMutableRawPointer {
        if(ptr == nil){
            ptr = Unmanaged.passRetained(glkView).toOpaque()
        }
        return ptr!
    }
    
    public var ignorePixelScaling: Bool = false {
        didSet {
            if(ignorePixelScaling){
                glkView.contentScaleFactor = 1
            }else {
                glkView.contentScaleFactor = UIScreen.main.scale
            }
        }
    }
    
    
    private(set) public var nativeGL: Int64 = 0
    private(set) public var nativeContext: Int64 = 0
    private(set) var native2DContext: Int64 = 0
    
    internal var glkView: CanvasGLKView
    private var is2D = false
    
    public var drawingBufferWidth: Int {
        return glkView.drawableWidth
    }
    public var drawingBufferHeight: Int {
        return glkView.drawableHeight
    }
    public var width: Float {
        get {
            return Float(glkView.frame.size.width)
        }
    }
    public var height: Float {
        get {
            return Float(glkView.frame.size.height)
        }
    }
    
    @objc public func initContext(
        _ type: String,
        _ alpha: Bool = true,
        _ antialias: Bool = true,
        _ depth: Bool = true,
        _ failIfMajorPerformanceCaveat: Bool = false,
        _ powerPreference: String = "default",
        _ premultipliedAlpha: Bool = true,
        _ preserveDrawingBuffer: Bool = false,
        _ stencil: Bool = false,
        _ desynchronized: Bool = false,
        _ xrCompatible: Bool = false
    ) {
       
        initContextWithContextAttributes(
            type,
            alpha,
            antialias,
            depth,
            failIfMajorPerformanceCaveat,
            powerPreference,
            premultipliedAlpha,
            preserveDrawingBuffer,
            stencil,
            desynchronized,
            xrCompatible
        )
    }
    
    
    
    func initContextWithContextAttributes(
        _ type: String,
        _ alpha: Bool,
        _ antialias: Bool,
        _ depth: Bool,
        _ failIfMajorPerformanceCaveat: Bool,
        _ powerPreference: String,
        _ premultipliedAlpha: Bool,
        _ preserveDrawingBuffer: Bool,
        _ stencil: Bool,
        _ desynchronized: Bool,
        _ xrCompatible: Bool
    ) {
        if (nativeGL != 0) {
            return
        }
        var version = -1
        var isCanvas = false
        switch (type) {
        case "2d":
            version = 0
            isCanvas = true
            is2D = isCanvas
            break
        case "experimental-webgl", "webgl":
            version = 1
            break
        case "webgl2":
            version = 2
            break
        default:
            break
        }
        
        if (version == -1) {
            return
        }
        
        
        var properties: [String: Any] = [:]
        let useWebGL = !isCanvas
        if(useWebGL && preserveDrawingBuffer){
            properties[kEAGLDrawablePropertyRetainedBacking] = NSNumber(value: preserveDrawingBuffer)
        }
        
        if(alpha){
            properties[kEAGLDrawablePropertyColorFormat] = kEAGLColorFormatRGBA8
            isOpaque = false
            glkView.isOpaque = false
            (glkView.layer as! CAEAGLLayer).isOpaque = false
        }else {
            properties[kEAGLDrawablePropertyColorFormat] = kEAGLColorFormatRGB565
            isOpaque = true
            (glkView.layer as! CAEAGLLayer).isOpaque = true
            glkView.isOpaque = true
        }
        
        
        if(!properties.isEmpty){
            let eaglLayer = self.glkView.layer as! CAEAGLLayer
            eaglLayer.drawableProperties = properties
        }
        
        if(useWebGL && depth){
            glkView.drawableDepthFormat = .format24
        }else {
            glkView.drawableDepthFormat = .formatNone
        }
        
        if(useWebGL && stencil){
            glkView.drawableStencilFormat = .format8
        }else if(isCanvas) {
            glkView.drawableStencilFormat = .format8
        }
        
        // antialias fails in 2D
        if(useWebGL && antialias){
             glkView.drawableMultisample = .multisample4X
        }
        
        let viewPtr = Int64(Int(bitPattern: getViewPtr()))
        
        
        let shared_context = NSCCanvas.shared_context
        
        nativeGL = CanvasHelpers.initSharedGLWithView(viewPtr,alpha, antialias, depth, failIfMajorPerformanceCaveat, type, premultipliedAlpha, preserveDrawingBuffer, stencil, desynchronized, xrCompatible, Int32(version), isCanvas, shared_context)

        
        // get new fbo

        nativeContext = CanvasHelpers.getGLPointer(nativeGL)
    }
    
    @objc public func create2DContext(
           _ alpha: Bool,
           _ antialias: Bool,
           _ depth: Bool,
           _ failIfMajorPerformanceCaveat: Bool,
           _ powerPreference: String,
           _ premultipliedAlpha: Bool,
           _ preserveDrawingBuffer: Bool,
           _ stencil: Bool,
           _ desynchronized: Bool,
           _ xrCompatible: Bool,
           _ fontColor: Int32
       ) -> Int64 {
           
           if(native2DContext != 0){
               return native2DContext
           }

           initContext(
               "2d",
               alpha,
               antialias,
               depth,
               failIfMajorPerformanceCaveat,
               powerPreference,
               premultipliedAlpha,
               preserveDrawingBuffer,
               stencil,
               desynchronized,
               xrCompatible
           )
           
           // disable for now
           let samples: Int32 = antialias ? 0 : 0
    
           glViewport(0, 0, GLsizei(drawingBufferWidth), GLsizei(drawingBufferHeight))
           
           var density = Float(UIScreen.main.scale)
           
           
           if (ignorePixelScaling) {
               density = 1
           }
           
           native2DContext = CanvasHelpers.create2DContext(
            nativeGL, Int32(drawingBufferWidth), Int32(drawingBufferHeight),
            alpha, density, samples, fontColor, density * 160, 0
           )
           return native2DContext
       }

        
    
    
    public func forceLayout(_ width: CGFloat, _ height: CGFloat){
        if (width == .zero && height == .zero) {
            return
        }
        
        frame = CGRect(x: frame.origin.x, y: frame.origin.y, width: width, height: height)
        glkView.frame = bounds
        
        setNeedsLayout()
        layoutIfNeeded()
    }
    
    
    public func snapshot(_ flip: Bool) -> UIImage?{
            if(is2D){
                CanvasHelpers.flush2DContext(native2DContext)
            }else {
                if(nativeGL != 0){
                    glkView.display()
                }
            }
        let snapshot = glkView.snapshot
        if(flip){
            return snapshot.withHorizontallyFlippedOrientation()
        }
        return snapshot
    }
    
    
    var renderer: NSCRender? = nil
    @discardableResult public func render() -> Bool{
        return CanvasHelpers.flushGL(nativeGL)
    }
    
    
    public func context2DTest(_ context: Int64){
        CanvasHelpers.test2D(context)
        render()
    }
    
    public func context2DTestToDataURL(_ context: Int64) -> String{
        return CanvasHelpers.testToDataURL(context)
    }
    private var enterBackgroundNotification: Any?
    private var becomeActiveNotification: Any?
    var enterBackgroundListener: (()-> Void)?
    var becomeActiveListener: (()-> Void)?
    private func setup(){
        self.enterBackgroundNotification = NotificationCenter.default.addObserver(forName: UIApplication.didEnterBackgroundNotification, object: nil, queue: nil){ _ in
            self.enterBackgroundListener?()
      }
        
        self.becomeActiveNotification = NotificationCenter.default.addObserver(forName: UIApplication.didBecomeActiveNotification, object: nil, queue: nil) { _ in
            self.becomeActiveListener?()
        }
    }
    
    private var handler: NSCTouchHandler?
    
    public var touchEventListener: ((String, UIGestureRecognizer) -> Void)?
    
    required init?(coder: NSCoder) {
        glkView = CanvasGLKView(coder: coder)!
        super.init(coder: coder)
        handler = NSCTouchHandler(canvas: self)
        backgroundColor = .clear
        glkView.enableSetNeedsDisplay = false
        glkView.contentScaleFactor = UIScreen.main.scale
        addSubview(glkView)
        self.isOpaque = false
        addGestureRecognizer(handler!.gestureRecognizer!)
        // addGestureRecognizer(handler!.pinchRecognizer!)
    }
    
    public override init(frame: CGRect) {
        glkView = CanvasGLKView(frame: frame)
        super.init(frame: frame)
        handler = NSCTouchHandler(canvas: self)
        backgroundColor = .clear
        glkView.enableSetNeedsDisplay = false
        glkView.contentScaleFactor = UIScreen.main.scale
        addSubview(glkView)
        self.isOpaque = false
        addGestureRecognizer(handler!.gestureRecognizer!)
       // addGestureRecognizer(handler!.pinchRecognizer!)
    }
    
    var ignoreTouchEvents = false {
        didSet {
            if(ignoreTouchEvents){
                removeGestureRecognizer(handler!.gestureRecognizer!)
             //   removeGestureRecognizer(handler!.pinchRecognizer!)
            }else {
                addGestureRecognizer(handler!.gestureRecognizer!)
               // addGestureRecognizer(handler!.pinchRecognizer!)
            }
        }
    }
    
    
    var readyListener: NSCCanvasListener?
    public func setListener(_ listener: NSCCanvasListener?){
        readyListener = listener
    }
    
    
    private var lastSize: CGRect = .null
    private var isLoaded: Bool = false
    
    private func resize(){
        if(nativeGL == 0){return}
        EAGLContext.setCurrent(glkView.context)
        if(is2D){
            glViewport(0, 0, GLsizei(drawingBufferWidth), GLsizei(drawingBufferHeight))
            CanvasHelpers.resize2DContext(native2DContext, Float(drawingBufferWidth), Float(drawingBufferHeight))
        }
    }
    
    public override func layoutSubviews() {
        if(bounds.isEmpty && lastSize.isEmpty){return}
        glkView.frame = bounds
        
        if(!isLoaded && drawingBufferWidth > 0 && drawingBufferHeight > 0){
            self.isLoaded = true
            self.readyListener?.contextReady()
        }
        
        if(drawingBufferWidth == 0 && drawingBufferHeight == 0){
            if(!isLoaded && (lastSize.width != .zero && lastSize.height != .zero)){
                self.isLoaded = true
                self.readyListener?.contextReady()
            }
        }else if(isLoaded && nativeGL != 0) {
            resize()
        }
        
        lastSize = bounds
    }
    
    
    
    deinit {
        if(nativeContext != 0){
            CanvasHelpers.releaseGLPointer(nativeContext)
            nativeContext = 0
        }
        if(nativeGL != 0){
            CanvasHelpers.releaseGL(nativeGL)
            nativeGL = 0
        }
        
        if(ptr != nil){
           let _ = Unmanaged<AnyObject>.fromOpaque(ptr!).takeRetainedValue()
        }
        
        if(enterBackgroundNotification != nil){
            NotificationCenter.default.removeObserver(enterBackgroundNotification!)
        }
        
        if(becomeActiveNotification != nil){
            NotificationCenter.default.removeObserver(becomeActiveNotification!)
        }
    }
    
    @objc public static func getBoundingClientRect(_ canvas: NSCCanvas, _ buffer: UnsafeMutableRawPointer) {
        var bytes = buffer.assumingMemoryBound(to: Float.self)
        let x = Float(canvas.frame.origin.x)
        let y = Float(canvas.frame.origin.y)
        let width = Float(canvas.frame.size.width)
        let height = Float(canvas.frame.size.height)
        bytes.pointee = y
        (bytes + 1).pointee = x + width
        (bytes + 2).pointee = y + height
        (bytes + 3).pointee = x
        (bytes + 4).pointee = width
        (bytes + 5).pointee = height
        (bytes + 6).pointee = x
        (bytes + 7).pointee = y
    }
    
}

extension String {
    subscript(_ range: CountableRange<Int>) -> String {
        let idx1 = index(startIndex, offsetBy: max(0, range.lowerBound))
        let idx2 = index(startIndex, offsetBy: min(self.count, range.upperBound))
        return String(self[idx1..<idx2])
    }
}
