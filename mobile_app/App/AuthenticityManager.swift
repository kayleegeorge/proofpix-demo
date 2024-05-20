//
//  AuthenticityManager.swift
//  ProofPix
//
//  Created by Sofiane Larbi on 5/17/24.
//

import Foundation

class AuthenticityManager {
    
    private let appAttestManager = AppAttestManager.shared;
    private let persistenceManager = PersistenceController.shared;
    
    static let shared = AuthenticityManager();

    func setup() {
        assert(appAttestManager.isReady())
        let url = URL(string: "https://appattest-demo.onrender.com/")!;
        let request = MultipartFormDataRequest(url: url)
        URLSession.shared.dataTask(with: request, completionHandler: {data,response,error in
            if error != nil {
                print("Error retrieving challenge!")
                return
            }
            self.appAttestManager.attestKey(hash: data!) { attestation in
                self.persistenceManager.saveAttestation(attestation: attestation)
                let url = URL(string: "https://appattest-demo.onrender.com/")!;
                let request = MultipartFormDataRequest(url: url)
                request.addTextField(named: "attestaion", value: attestation.base64EncodedString())
                URLSession.shared.dataTask(with: request, completionHandler: {data,response,error in
                    print("Callback...")
                    if error != nil {
                        print("Error!")
                        return
                    }
                    PersistenceController.shared.setAttested()
                }).resume()
            }
        }).resume()
    }
    
    func setupIfNeeded() {
        if !persistenceManager.isAttested() { self.setup() }
    }
}
