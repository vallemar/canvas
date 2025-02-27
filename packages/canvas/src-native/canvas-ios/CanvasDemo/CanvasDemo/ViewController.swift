import UIKit
import CanvasNative
import GLKit
@available(iOS 13.0, *)
class ViewController: UIViewController {
    func contextReady() {
        print("ready")
    }
    
    var canvas: NSCCanvas?
    var imageView: UIImageView?
    let PI: Float = .pi
    let TWO_PI: Float = .pi * 2
    var tapped = 0
    
    override init(nibName nibNameOrNil: String?, bundle nibBundleOrNil: Bundle?) {
        super.init(nibName: nibNameOrNil, bundle: nibBundleOrNil)
    }
    
    required init?(coder: NSCoder) {
        canvas = NSCCanvas()
        super.init(coder: coder)
         view.addSubview(canvas!)
    }
    
   // var svg: TNSSVG?
    override func viewDidLoad() {
        super.viewDidLoad()
        view.backgroundColor = .white
        canvas?.forceLayout(view.bounds.width, view.bounds.height)
       // canvas = NSCCanvas(frame: view.bounds)
        //view.addSubview(canvas!)
//        canvas?.frame = view.frame
//        canvas?.layoutIfNeeded()
        
       let ctx = canvas!.create2DContext(
                                    true,
                                    true,
                                    true,
                                    false,
                                    "default",
                                    true,
                                    false,
                                    false,
                                    false,
                                    false,
                                    -16777216
        )
        
        let glView1 = canvas!.subviews.first as? GLKView
        


        canvas?.context2DTest(ctx)
        
        
        
        let canvas2 = NSCCanvas(frame: .zero)
        
        canvas2.forceLayout(300, 150)
        

        let ctx2 = canvas2.create2DContext(
                                    true,
                                    true,
                                    true,
                                    false,
                                    "default",
                                    true,
                                    false,
                                    false,
                                    false,
                                    false,
                                    -16777216
        )
            
  
        canvas2.forceLayout(300, 300)

            
        
         var vp = Array(repeating: GLint(), count: 4)

         glGetIntegerv(GLenum(GL_VIEWPORT), &vp)

         print(vp)
        

       canvas2.context2DTest(ctx2)
//
       let glView = canvas2.subviews.first as? GLKView
//
        let data = glView?.snapshot.pngData()?.base64EncodedData()
       
        if(data != nil){
            print(String(data: data!, encoding: .utf8))
        }
        
        /*var start = CACurrentMediaTime()
        print(canvas?.context2DTestToDataURL(ctx))
        print("done", CACurrentMediaTime() - start)
        
        
     
        
        start = CACurrentMediaTime()
        let data = glView.snapshot.pngData()?.base64EncodedData()
       
        print("done", CACurrentMediaTime() - start)
        */
//        print(String(data: data!, encoding: .utf8))
        
    //    print(canvas?.context2DTestToDataURL(ctx))
        // Do any additional setup after loading the view.
       //canvas1.setListener(self)
        
       // canvas1?.ignorePixelScaling = true
       // canvas1.backgroundColor = .white
       
    
    }
    
    
    func drawAll() {
 
        
    }
    
   
}
