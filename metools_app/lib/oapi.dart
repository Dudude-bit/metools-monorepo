import 'dart:io';

import 'package:openapi_generator_annotations/openapi_generator_annotations.dart';

const swaggerPath = Platform.environment["SWAGGER_PATH"]

@Openapi(
  additionalProperties:
  DioProperties(pubName: 'me_api'),
  inputSpec:
  RemoteSpec(path: swaggerPath),
  generatorName: Generator.dio,
  runSourceGenOnOutput: true,
  outputDirectory: 'api/me_api',
)
class Example {}