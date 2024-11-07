import SwiftUI
import RealmSwift

let realmApp = RealmSwift.App(id: "vercre-wallet-xxxxxx") // TODO: Is this needed?

@main
struct VercreWalletApp: SwiftUI.App {
    var body: some Scene {
        WindowGroup {
            ContentView(core: Core())
        }
    }
}
