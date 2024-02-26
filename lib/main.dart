import 'package:flutter/material.dart';
import 'package:img_send/src/rust/api/simple.dart';
import 'package:img_send/src/rust/frb_generated.dart';

Future<void> main() async {
    await RustLib.init();
    runApp(const MyApp());

    // for (var i = 0; i < 10; ++i) {
    // final tt = Stopwatch()..start();
    // await renderImage(width: 3000, height: 3000);
    // tt.stop();
    // print(' => IN FLUTTER: ${tt.elapsedMicroseconds} us');
    // }

}

class MyApp extends StatelessWidget {
  const MyApp({super.key});
  @override
  Widget build(BuildContext context) {

    var tt = Stopwatch()..start();
    Future<BMPimage> bmpImg = renderImage(width: 3000, height: 3000); //test with some big image
    // var imageWidget = Image.memory(bmpImg.bmp);
    var imageWidget = FutureBuilder<BMPimage>(
        future: bmpImg,
        builder: (BuildContext context, AsyncSnapshot<BMPimage> snapshot) {
        if (snapshot.connectionState == ConnectionState.done) {
            if (snapshot.hasError) {
            print('Error: ${snapshot.error}');
            return Text('Error: ${snapshot.error}');
            } else {
                print(' => IN FLUTTER: ${tt.elapsedMilliseconds} ms');
            return Image.memory(snapshot.data!.bmp);
            }
        } else {
            return CircularProgressIndicator();
        }
        },
    );


    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: Center(
        //   child: Text('Hello, World!'),
          child: imageWidget,
        ),
      ),
    );
  }
}
