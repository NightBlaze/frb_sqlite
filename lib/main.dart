import 'dart:io';

import 'package:flutter/material.dart';
import 'package:frb_sqlite/src/rust/api/simple.dart';
import 'package:frb_sqlite/src/rust/frb_generated.dart';
import 'package:path_provider/path_provider.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: Column(
          children: [
            Center(
              child: Text('Action: Call Rust `greet("Tom")`\nResult: `${greet(name: "Tom")}`'),
            ),
            TextButton(
                onPressed: () async {
                  Directory? documentsDirectory;
                  try {
                    documentsDirectory = await getApplicationDocumentsDirectory();
                  } catch (err) {
                    print('Cant get documents directory');
                  }

                  configureDatabase(documentsDir: '${documentsDirectory?.path}');
                },
                child: const Text("DB"))
          ],
        ),
      ),
    );
  }
}
