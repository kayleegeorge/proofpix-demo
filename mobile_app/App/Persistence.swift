//
//  Persistence.swift
//  ProofPix
//
//  Created by Sofiane Larbi on 2/16/24.
//

import CoreData

struct PersistenceController {
    static let shared = PersistenceController()
    
    let container: NSPersistentContainer
    let defaults = UserDefaults.standard
    
    init(inMemory: Bool = false) {
        container = NSPersistentContainer(name: "ProofPix")
        if inMemory {
            container.persistentStoreDescriptions.first!.url = URL(fileURLWithPath: "/dev/null")
        }
        container.loadPersistentStores(completionHandler: { (storeDescription, error) in
            if let error = error as NSError? {
                fatalError("Unresolved error \(error), \(error.userInfo)")
            }
        })
        container.viewContext.automaticallyMergesChangesFromParent = true
    }
        
    // TODO: fix this
    func saveURL(url: String) {
        let context = container.viewContext
//        let pic = ProvedPic(context: context)
//        pic.url = url
        do {
            try context.save()
        } catch {
            let nsError = error as NSError
            fatalError("Unresolved error \(nsError), \(nsError.userInfo)")
        }
    }
    
//    func fetchURLs() -> [String] {
//        let context = container.viewContext
//        let fetchRequest: NSFetchRequest<ProvedPic> = ProvedPic.fetchRequest()
//        do {
//            let images = try context.fetch(fetchRequest)
//            return images.map { $0.url! }
//        } catch {
//            let nsError = error as NSError
//            fatalError("Unresolved error \(nsError), \(nsError.userInfo)")
//        }
//    }
    
    func saveAttestation(attestation: Data) {
        defaults.set(attestation, forKey: "attestation")
    }
    
    func saveKeyId(keyId: Data) {
        defaults.set(keyId, forKey: "keyId")
    }
    
    func setAttested() {
        defaults.set(true, forKey: "attested")
    }
    
    func isAttested() -> Bool {
        return defaults.bool(forKey: "attested")
    }
}
