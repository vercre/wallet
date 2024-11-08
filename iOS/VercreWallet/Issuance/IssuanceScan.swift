//
//  SwiftUIView.swift
//  VercreWallet
//
//  Created by Andrew Goldie on 21/10/2024.
//

import SwiftUI
import CodeScanner

struct IssuanceScan: View {
    @ObservedObject var core: Core
    @State private var scannerVisible = false
    @State var scanResult: String = "Scan a Verifiable Credential offer QR code"
    
    init(core: Core) {
        self.core = core
    }
    
    var scannerSheet: some View {
        CodeScannerView(
            codeTypes: [.qr],
            simulatedData: "openid-credential-offer://credential_offer=wibble",
            completion: handleScan
        )
    }

    var body: some View {
        VStack(spacing: 48) {
            Text(scanResult)
            Button("Scan Offer", systemImage: "qrcode.viewfinder") {
                self.scannerVisible = true
            }
            .buttonStyle(.borderedProminent)
            .tint(.blue)
            .sheet(isPresented: $scannerVisible) {
                self.scannerSheet
            }
        }
    }
    
    func handleScan(result: Result<ScanResult, ScanError>) {
        switch result {
        case .success(let code):
            self.scanResult = "Offer scanned"
            let parts = code.string.components(separatedBy: "credential_offer=")
            guard parts.count == 2 else {
                self.scanResult = "Invalid QR code"
                self.scannerVisible = false
                return
            }
            let offer = parts[1]
            // TODO: send the offer back to Crux
            debugPrint("Offer: \(offer)")
        case .failure(let error):
            debugPrint(error.localizedDescription)
            self.scanResult = "Failed to scan QR code"
        }
        self.scannerVisible = false
            
    }
}

#Preview {
    IssuanceScan(core: Core())
}
