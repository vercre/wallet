//
//  ImageViewModel.swift
//  VercreWallet
//
//  Created by Andrew Goldie on 21/11/2024.
//

import Foundation
import SwiftUI

class ImageViewModel: ObservableObject {
    @Published var image: UIImage?
    
    private var imageCache: NSCache<NSString, UIImage>?
}
