import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:test/test.dart';

import 'helpers.dart';

void main() {
  late BindingLiquidSdk sdk;

  group('main', () {
    setUpAll(() async {
      await initApi();
      ConnectRequest connectRequest = ConnectRequest(mnemonic: "", config: defaultConfig(network: Network.testnet));
      sdk = await connect(req: connectRequest);
    });

    test("after setting up, getInfo should throw exception with 'Not initialized' message", () async {
      GetInfoRequest getInfoRequest = GetInfoRequest(withScan: true);
      try {
        await sdk.getInfo(req: getInfoRequest);
      } catch (e) {
        if (e is AnyhowException) {
          expect(e.message, "Not initialized");
        }
      }
    });
  });
}
