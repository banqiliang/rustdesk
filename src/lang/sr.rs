lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Status"),
        ("Your Desktop", "Vaša radna površina"),
        ("desk_tip", "Vašoj radnoj površini se može pristupiti ovim ID i lozinkom."),
        ("Password", "Lozinka"),
        ("Ready", "Spremno"),
        ("Established", "Uspostavljeno"),
        ("connecting_status", "Spajanje na RustDesk mrežu..."),
        ("Enable Service", "Dozvoli servis"),
        ("Start Service", "Pokreni servis"),
        ("Service is running", "Servis je pokrenut"),
        ("Service is not running", "Servis nije pokrenut"),
        ("not_ready_status", "Nije spremno. Proverite konekciju."),
        ("Control Remote Desktop", "Upravljanje udaljenom radnom površinom"),
        ("Transfer File", "Prenos fajla"),
        ("Connect", "Spajanje"),
        ("Recent Sessions", "Poslednje sesije"),
        ("Address Book", "Adresar"),
        ("Confirmation", "Potvrda"),
        ("TCP Tunneling", "TCP tunel"),
        ("Remove", "Ukloni"),
        ("Refresh random password", "Osveži slučajnu lozinku"),
        ("Set your own password", "Postavi lozinku"),
        ("Enable Keyboard/Mouse", "Dozvoli tastaturu/miša"),
        ("Enable Clipboard", "Dozvoli clipboard"),
        ("Enable File Transfer", "Dozvoli prenos fajlova"),
        ("Enable TCP Tunneling", "Dozvoli TCP tunel"),
        ("IP Whitelisting", "IP pouzdana lista"),
        ("ID/Relay Server", "ID/Posredni server"),
        ("Import Server Config", "Import server konfiguracije"),
        ("Export Server Config", "Eksport server konfiguracije"),
        ("Import server configuration successfully", "Import server konfiguracije uspešan"),
        ("Export server configuration successfully", "Eksport server konfiguracije uspešan"),
        ("Invalid server configuration", "Pogrešna konfiguracija servera"),
        ("Clipboard is empty", "Clipboard je prazan"),
        ("Stop service", "Zaustavi servis"),
        ("Change ID", "Promeni ID"),
        ("Website", "Web sajt"),
        ("About", "O programu"),
        ("About RustDesk", ""),
        ("Slogan_tip", ""),
        ("Privacy Statement", ""),
        ("Mute", "Utišaj"),
        ("Audio Input", "Audio ulaz"),
        ("Enhancements", "Proširenja"),
        ("Hardware Codec", "Hardverski kodek"),
        ("Adaptive Bitrate", "Prilagodljiva gustina podataka"),
        ("ID Server", "ID server"),
        ("Relay Server", "Posredni server"),
        ("API Server", "API server"),
        ("invalid_http", "Nevažeći http"),
        ("Invalid IP", "Nevažeća IP"),
        ("id_change_tip", "Dozvoljeni su samo a-z, A-Z, 0-9 i _ (donja crta) znakovi. Prvi znak mora biti slovo a-z, A-Z. Dužina je od 6 do 16."),
        ("Invalid format", "Pogrešan format"),
        ("server_not_support", "Server nije podržan"),
        ("Not available", "Nije dostupno"),
        ("Too frequent", "Previše često"),
        ("Cancel", "Otkaži"),
        ("Skip", "Preskoči"),
        ("Close", "Zatvori"),
        ("Retry", "Ponovi"),
        ("OK", "Ok"),
        ("Password Required", "Potrebna lozinka"),
        ("Please enter your password", "Molimo unesite svoju lozinku"),
        ("Remember password", "Zapamti lozinku"),
        ("Wrong Password", "Pogrešna lozinka"),
        ("Do you want to enter again?", "Želite li da unesete ponovo?"),
        ("Connection Error", "Greška u konekciji"),
        ("Error", "Greška"),
        ("Reset by the peer", "Prekinuto sa druge strane"),
        ("Connecting...", "Povezivanje..."),
        ("Connection in progress. Please wait.", "Povezivanje u toku. Molimo sačekajte."),
        ("Please try 1 minute later", "Pokušajte minut kasnije"),
        ("Login Error", "Greška u prijavljivanju"),
        ("Successful", "Uspešno"),
        ("Connected, waiting for image...", "Spojeno, sačekajte sliku..."),
        ("Name", "Ime"),
        ("Type", "Tip"),
        ("Modified", "Izmenjeno"),
        ("Size", "Veličina"),
        ("Show Hidden Files", "Prikaži skrivene datoteke"),
        ("Receive", "Prijem"),
        ("Send", "Slanje"),
        ("Refresh File", "Osveži datoteku"),
        ("Local", "Lokalno"),
        ("Remote", "Udaljeno"),
        ("Remote Computer", "Udaljeni računar"),
        ("Local Computer", "Lokalni računar"),
        ("Confirm Delete", "Potvrdite brisanje"),
        ("Delete", "Brisanje"),
        ("Properties", "Osobine"),
        ("Multi Select", "Višestruko selektovanje"),
        ("Select All", "Selektuj sve"),
        ("Unselect All", "Deselektuj sve"),
        ("Empty Directory", "Prazan direktorijum"),
        ("Not an empty directory", "Nije prazan direktorijum"),
        ("Are you sure you want to delete this file?", "Da li ste sigurni da želite da obrišete ovu datoteku?"),
        ("Are you sure you want to delete this empty directory?", "Da li ste sigurni da želite da obrišete ovaj prazan direktorijum?"),
        ("Are you sure you want to delete the file of this directory?", "Da li ste sigurni da želite da obrišete datoteku ovog direktorijuma?"),
        ("Do this for all conflicts", "Uradi ovo za sve konflikte"),
        ("This is irreversible!", "Ovo je nepovratno"),
        ("Deleting", "Brisanje"),
        ("files", "datoteke"),
        ("Waiting", "Čekanje"),
        ("Finished", "Završeno"),
        ("Speed", "Brzina"),
        ("Custom Image Quality", "Korisnički kvalitet slike"),
        ("Privacy mode", "Mod privatnosti"),
        ("Block user input", "Blokiraj korisnikov unos"),
        ("Unblock user input", "Odblokiraj korisnikov unos"),
        ("Adjust Window", "Podesi prozor"),
        ("Original", "Original"),
        ("Shrink", "Skupi"),
        ("Stretch", "Raširi"),
        ("Scrollbar", "Skrol linija"),
        ("ScrollAuto", "Auto skrol"),
        ("Good image quality", "Dobar kvalitet slike"),
        ("Balanced", "Balansirano"),
        ("Optimize reaction time", "Optimizuj vreme reakcije"),
        ("Custom", "Korisnički"),
        ("Show remote cursor", "Prikaži udaljeni kursor"),
        ("Show quality monitor", "Prikaži monitor kvaliteta"),
        ("Disable clipboard", "Zabrani clipboard"),
        ("Lock after session end", "Zaključaj po završetku sesije"),
        ("Insert", "Umetni"),
        ("Insert Lock", "Zaključaj umetanje"),
        ("Refresh", "Osveži"),
        ("ID does not exist", "ID ne postoji"),
        ("Failed to connect to rendezvous server", "Greška u spajanju na server za povezivanje"),
        ("Please try later", "Molimo pokušajte kasnije"),
        ("Remote desktop is offline", "Udaljeni ekran je isključen"),
        ("Key mismatch", "Pogrešan ključ"),
        ("Timeout", "Isteklo vreme"),
        ("Failed to connect to relay server", "Greška u spajanju na posredni server"),
        ("Failed to connect via rendezvous server", "Greška u spajanju preko servera za povezivanje"),
        ("Failed to connect via relay server", "Greška u spajanju preko posrednog servera"),
        ("Failed to make direct connection to remote desktop", "Greška u direktnom spajanju na udaljenu radnu površinu"),
        ("Set Password", "Postavi lozinku"),
        ("OS Password", "OS lozinka"),
        ("install_tip", "Zbog UAC RustDesk ne može raditi pravilno u nekim slučajevima. Da biste prevazišli UAC, kliknite taster ispod da instalirate RustDesk na sistem."),
        ("Click to upgrade", "Klik za nadogradnju"),
        ("Click to download", "Klik za preuzimanje"),
        ("Click to update", "Klik za ažuriranje"),
        ("Configure", "Konfigurisanje"),
        ("config_acc", "Da biste daljinski kontrolisali radnu površinu, RustDesk-u treba da dodelite \"Accessibility\" prava."),
        ("config_screen", "Da biste daljinski pristupili radnoj površini, RustDesk-u treba da dodelite \"Screen Recording\" prava."),
        ("Installing ...", "Instaliranje..."),
        ("Install", "Instaliraj"),
        ("Installation", "Instalacija"),
        ("Installation Path", "Putanja za instalaciju"),
        ("Create start menu shortcuts", "Kreiraj prečice u meniju"),
        ("Create desktop icon", "Kreiraj ikonicu na radnoj površini"),
        ("agreement_tip", "Pokretanjem instalacije prihvatate ugovor o licenciranju."),
        ("Accept and Install", "Prihvati i instaliraj"),
        ("End-user license agreement", "Ugovor sa krajnjim korisnikom"),
        ("Generating ...", "Generisanje..."),
        ("Your installation is lower version.", "Vaša instalacija je niže verzije"),
        ("not_close_tcp_tip", "Ne zatvarajte ovaj prozor dok koristite tunel"),
        ("Listening ...", "Na slušanju..."),
        ("Remote Host", "Adresa udaljenog uređaja"),
        ("Remote Port", "Udaljeni port"),
        ("Action", "Akcija"),
        ("Add", "Dodaj"),
        ("Local Port", "Lokalni port"),
        ("Local Address", "Lokalna adresa"),
        ("Change Local Port", "Promeni lokalni port"),
        ("setup_server_tip", "Za brže spajanje, molimo da koristite svoj server"),
        ("Too short, at least 6 characters.", "Prekratko, najmanje 6 znakova."),
        ("The confirmation is not identical.", "Potvrda nije identična"),
        ("Permissions", "Dozvole"),
        ("Accept", "Prihvati"),
        ("Dismiss", "Odbaci"),
        ("Disconnect", "Raskini konekciju"),
        ("Allow using keyboard and mouse", "Dozvoli korišćenje tastature i miša"),
        ("Allow using clipboard", "Dozvoli korišćenje clipboard-a"),
        ("Allow hearing sound", "Dozvoli da se čuje zvuk"),
        ("Allow file copy and paste", "Dozvoli kopiranje i lepljenje fajlova"),
        ("Connected", "Spojeno"),
        ("Direct and encrypted connection", "Direktna i kriptovana konekcija"),
        ("Relayed and encrypted connection", "Posredna i kriptovana konekcija"),
        ("Direct and unencrypted connection", "Direktna i nekriptovana konekcija"),
        ("Relayed and unencrypted connection", "Posredna i nekriptovana konekcija"),
        ("Enter Remote ID", "Unesite ID udaljenog uređaja"),
        ("Enter your password", "Unesite svoju lozinku"),
        ("Logging in...", "Prijava..."),
        ("Enable RDP session sharing", "Dozvoli deljenje RDP sesije"),
        ("Auto Login", "Auto prijavljivanje (Važeće samo ako ste postavili \"Lock after session end\")"),
        ("Enable Direct IP Access", "Dozvoli direktan pristup preko IP"),
        ("Rename", "Preimenuj"),
        ("Space", "Prazno"),
        ("Create Desktop Shortcut", "Kreiraj prečicu na radnoj površini"),
        ("Change Path", "Promeni putanju"),
        ("Create Folder", "Kreiraj direktorijum"),
        ("Please enter the folder name", "Unesite ime direktorijuma"),
        ("Fix it", "Popravi ga"),
        ("Warning", "Upozorenje"),
        ("Login screen using Wayland is not supported", "Ekran za prijavu koji koristi Wayland nije podržan"),
        ("Reboot required", "Potreban je restart"),
        ("Unsupported display server ", "Nepodržan server za prikaz"),
        ("x11 expected", "x11 očekivan"),
        ("Port", "Port"),
        ("Settings", "Postavke"),
        ("Username", "Korisničko ime"),
        ("Invalid port", "Pogrešan port"),
        ("Closed manually by the peer", "Klijent ručno raskinuo konekciju"),
        ("Enable remote configuration modification", "Dozvoli modifikaciju udaljene konfiguracije"),
        ("Run without install", "Pokreni bez instalacije"),
        ("Always connected via relay", "Uvek spojne preko posrednika"),
        ("Always connect via relay", "Uvek se spoj preko posrednika"),
        ("whitelist_tip", "Samo dozvoljene IP mi mogu pristupiti"),
        ("Login", "Prijava"),
        ("Logout", "Odjava"),
        ("Tags", "Oznake"),
        ("Search ID", "Traži ID"),
        ("Current Wayland display server is not supported", "Tekući Wazland server za prikaz nije podržan"),
        ("whitelist_sep", "Odvojeno zarezima, tačka zarezima, praznim mestima ili novim redovima"),
        ("Add ID", "Dodaj ID"),
        ("Add Tag", "Dodaj oznaku"),
        ("Unselect all tags", "Odselektuj sve oznake"),
        ("Network error", "Greška na mreži"),
        ("Username missed", "Korisničko ime promašeno"),
        ("Password missed", "Lozinka promašena"),
        ("Wrong credentials", "Pogrešno korisničko ime ili lozinka"),
        ("invalid_http", "mora početi sa http:// ili https://"),
        ("Edit Tag", "Izmeni oznaku"),
        ("Unremember Password", "Zaboravi lozinku"),
        ("Favorites", "Favoriti"),
        ("Add to Favorites", "Dodaj u favorite"),
        ("Remove from Favorites", "Izbaci iz favorita"),
        ("Empty", "Prazno"),
        ("Invalid folder name", "Pogrešno ime direktorijuma"),
        ("Socks5 Proxy", "Socks5 proksi"),
        ("Hostname", "Ime uređaja"),
        ("Discovered", "Otkriveno"),
        ("install_daemon_tip", "Za pokretanje pri startu sistema, treba da instalirate sistemski servis."),
        ("Remote ID", "Udaljeni ID"),
        ("Paste", "Nalepi"),
        ("Paste here?", "Nalepi ovde?"),
        ("Are you sure to close the connection?", "Da li ste sigurni da zatvarate konekciju?"),
        ("Download new version", "Preuzmi novu verziju"),
        ("Touch mode", "Mod na dodir"),
        ("Mouse mode", "Miš mod"),
        ("One-Finger Tap", "Pritisak jednim prstom"),
        ("Left Mouse", "Levi miš"),
        ("One-Long Tap", "Dugi pritisak"),
        ("Two-Finger Tap", "Pritisak sa dva prsta"),
        ("Right Mouse", "Desni miš"),
        ("One-Finger Move", "Pomeranje jednim prstom"),
        ("Double Tap & Move", "Dupli pritisak i pomeranje"),
        ("Mouse Drag", "Prevlačenje mišem"),
        ("Three-Finger vertically", "Sa tri prsta vertikalno"),
        ("Mouse Wheel", "Točkić miša"),
        ("Two-Finger Move", "Pomeranje sa dva prsta"),
        ("Canvas Move", "Pomeranje pozadine"),
        ("Pinch to Zoom", "Stisnite za zumiranje"),
        ("Canvas Zoom", "Zumiranje pozadine"),
        ("Reset canvas", "Resetuj pozadinu"),
        ("No permission of file transfer", "Nemate pravo prenosa datoteka"),
        ("Note", "Primedba"),
        ("Connection", "Konekcija"),
        ("Share Screen", "Podeli ekran"),
        ("CLOSE", "ZATVORI"),
        ("OPEN", "OTVORI"),
        ("Chat", "Dopisivanje"),
        ("Total", "Ukupno"),
        ("items", "stavki"),
        ("Selected", "Izabrano"),
        ("Screen Capture", "Snimanje ekrana"),
        ("Input Control", "Kontrola unosa"),
        ("Audio Capture", "Snimanje zvuka"),
        ("File Connection", "Spajanje preko datoteke"),
        ("Screen Connection", "Podeli konekciju"),
        ("Do you accept?", "Prihvatate?"),
        ("Open System Setting", "Postavke otvorenog sistema"),
        ("How to get Android input permission?", "Kako dobiti pristup za Android unos?"),
        ("android_input_permission_tip1", "Da bi daljinski uređaj kontrolisao vaš Android uređaj preko miša ili na dodir, treba da dozvolite RustDesk-u da koristi \"Accessibility\" servis."),
        ("android_input_permission_tip2", "Molimo pređite na sledeću stranicu sistemskih podešavanja, pronađite i unesite [Installed Services], uključite [RustDesk Input] servis."),
        ("android_new_connection_tip", "Primljen je novi zahtev za upravljanje, koji želi da upravlja ovim vašim uređajem."),
        ("android_service_will_start_tip", "Uključenje \"Screen Capture\" automatski će pokrenuti servis, dozvoljavajući drugim uređajima da zahtevaju spajanje na vaš uređaj."),
        ("android_stop_service_tip", "Zatvaranje servisa automatski će zatvoriti sve uspostavljene konekcije."),
        ("android_version_audio_tip", "Tekuća Android verzija ne podržava audio snimanje, molimo nadogradite na Android 10 ili veći."),
        ("android_start_service_tip", "Kliknite [Start Service] ili OPEN [Screen Capture] dozvolu da pokrenete servis deljenja ekrana."),
        ("Account", "Nalog"),
        ("Overwrite", "Prepiši preko"),
        ("This file exists, skip or overwrite this file?", "Ova datoteka postoji, preskoči ili prepiši preko?"),
        ("Quit", "Izlaz"),
        ("doc_mac_permission", "https://rustdesk.com/docs/en/manual/mac/#enable-permissions"),
        ("doc_fix_wayland", "https://rustdesk.com/docs/en/manual/linux/#x11-required"),
        ("server_not_support", "Server još uvek ne podržava"),
        ("Help", "Pomoć"),
        ("Failed", "Greška"),
        ("Succeeded", "Uspešno"),
        ("Someone turns on privacy mode, exit", "Neko je uključio mod privatnosti, izlaz."),
        ("Unsupported", "Nepodržano"),
        ("Peer denied", "Klijent zabranjen"),
        ("Please install plugins", "Molimo instalirajte dodatke"),
        ("Peer exit", "Klijent izašao"),
        ("Failed to turn off", "Greška kod isključenja"),
        ("Turned off", "Isključeno"),
        ("In privacy mode", "U modu privatnosti"),
        ("Out privacy mode", "Van moda privatnosti"),
        ("Language", "Jezik"),
        ("Keep RustDesk background service", "Zadrži RustDesk kao pozadinski servis"),
        ("Ignore Battery Optimizations", "Zanemari optimizacije baterije"),
        ("android_open_battery_optimizations_tip", "Ako želite da onemogućite ovu funkciju, molimo idite na sledeću stranicu za podešavanje RustDesk aplikacije, pronađite i uđite u [Battery], isključite [Unrestricted]"),
        ("Connection not allowed", "Konekcija nije dozvoljena"),
        ("Legacy mode", "Zastareli mod"),
        ("Map mode", "Mod mapiranja"),
        ("Translate mode", "Mod prevođenja"),
        ("Use permanent password", "Koristi trajnu lozinku"),
        ("Use both passwords", "Koristi obe lozinke"),
        ("Set permanent password", "Postavi trajnu lozinku"),
        ("Enable Remote Restart", "Omogući daljinsko restartovanje"),
        ("Allow remote restart", "Dozvoli daljinsko restartovanje"),
        ("Restart Remote Device", "Restartuj daljinski uređaj"),
        ("Are you sure you want to restart", "Da li ste sigurni da želite restart"),
        ("Restarting Remote Device", "Restartovanje daljinskog uređaja"),
        ("remote_restarting_tip", "Udaljeni uređaj se restartuje, molimo zatvorite ovu poruku i ponovo se kasnije povežite trajnom šifrom"),
        ("Are you sure to close the connection?", "Da li ste sigurni da želite da zatvorite konekciju?"),
        ("Copied", "Kopirano"),
        ("Exit Fullscreen", "Napusti mod celog ekrana"),
        ("Fullscreen", "Mod celog ekrana"),
        ("Mobile Actions", "Mobilne akcije"),
        ("Select Monitor", "Izbor monitora"),
        ("Control Actions", "Upravljačke akcije"),
        ("Display Settings", "Postavke prikaza"),
        ("Ratio", "Odnos"),
        ("Image Quality", "Kvalitet slike"),
        ("Scroll Style", "Stil skrolovanja"),
        ("Show Menubar", "Prikaži meni"),
        ("Hide Menubar", "Sakrij meni"),
        ("Direct Connection", "Direktna konekcija"),
        ("Relay Connection", "Posredna konekcija"),
        ("Secure Connection", "Bezbedna konekcija"),
        ("Insecure Connection", "Nebezbedna konekcija"),
        ("Scale original", "Skaliraj original"),
        ("Scale adaptive", "Adaptivno skaliranje"),
        ("General", "Uopšteno"),
        ("Security", "Bezbednost"),
        ("Account", "Nalog"),
        ("Theme", "Tema"),
        ("Dark Theme", "Tamna tema"),
        ("Dark", "Tamno"),
        ("Light", "Svetlo"),
        ("Follow System", "Prema sistemu"),
        ("Enable hardware codec", "Omogući hardverski kodek"),
        ("Unlock Security Settings", "Otključaj postavke bezbednosti"),
        ("Enable Audio", "Dozvoli zvuk"),
        ("Unlock Network Settings", "Otključaj postavke mreže"),
        ("Server", "Server"),
        ("Direct IP Access", "Direktan IP pristup"),
        ("Proxy", "Proksi"),
        ("Port", "Port"),
        ("Apply", "Primeni"),
        ("Disconnect all devices?", "Otkači sve uređaju?"),
        ("Clear", "Obriši"),
        ("Audio Input Device", "Uređaj za ulaz zvuka"),
        ("Deny remote access", "Zabrani daljinski pristup"),
        ("Use IP Whitelisting", "Koristi listu pouzdanih IP"),
        ("Network", "Mreža"),
        ("Enable RDP", "Dozvoli RDP"),
        ("Pin menubar", "Zakači meni"),
        ("Unpin menubar", "Otkači meni"),
        ("Recording", "Snimanje"),
        ("Directory", "Direktorijum"),
        ("Automatically record incoming sessions", "Automatski snimaj dolazne sesije"),
        ("Change", "Promeni"),
        ("Start session recording", "Započni snimanje sesije"),
        ("Stop session recording", "Zaustavi snimanje sesije"),
        ("Enable Recording Session", "Omogući snimanje sesije"),
        ("Allow recording session", "Dozvoli snimanje sesije"),
        ("Enable LAN Discovery", "Omogući LAN otkrivanje"),
        ("Deny LAN Discovery", "Zabrani LAN otkrivanje"),
        ("Write a message", "Napiši poruku"),
        ("Prompt", "Prompt"),
        ("Please wait for confirmation of UAC...", "Molimo sačekajte UAC potvrdu..."),
        ("elevated_foreground_window_tip", "Tekući prozor udaljene radne površine zahteva veću privilegiju za rad, tako da trenutno nije moguće koristiti miša i tastaturu. Možete zahtevati od udaljenog korisnika da minimizira aktivni prozor, ili kliknuti na taster za podizanje privilegija u prozoru za rad sa konekcijom. Da biste prevazišli ovaj problem, preporučljivo je da instalirate softver na udaljeni uređaj."),
        ("Disconnected", "Odspojeno"),
        ("Other", "Ostalo"),
        ("Confirm before closing multiple tabs", "Potvrda pre zatvaranja više kartica"),
        ("Keyboard Settings", "Postavke tastature"),
        ("Custom", "Korisnički"),
        ("Full Access", "Pun pristup"),
        ("Screen Share", "Deljenje ekrana"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland zahteva Ubuntu 21.04 ili veću verziju"),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland zahteva veću verziju Linux distribucije. Molimo pokušajte X11 ili promenite OS."),
        ("JumpLink", "Vidi"),
        ("Stop service", "Stopiraj servis"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Molimo izaberite ekran koji će biti podeljen (Za rad na klijent strani)"),
        ("Show RustDesk", "Prikazi RustDesk"),
        ("This PC", "Ovaj PC"),
        ("or", "ili"),
        ("Continue with", "Nastavi sa"),
        ("Elevate", "Izdigni"),
        ("Zoom cursor", "Zumiraj kursor"),
        ("Accept sessions via password", "Prihvati sesije preko lozinke"),
        ("Accept sessions via click", "Prihvati sesije preko klika"),
        ("Accept sessions via both", "Prihvati sesije preko oboje"),
        ("Please wait for the remote side to accept your session request...", "Molimo sačekajte da udaljena strana prihvati vaš zahtev za sesijom..."),
        ("One-time Password", "Jednokratna lozinka"),
        ("Use one-time password", "Koristi jednokratnu lozinku"),
        ("One-time password length", "Dužina jednokratne lozinke"),
        ("Request access to your device", "Zahtev za pristup vašem uređaju"),
        ("Hide connection management window", "Sakrij prozor za uređivanje konekcije"),
        ("hide_cm_tip", "Skrivanje dozvoljeno samo prihvatanjem sesije preko lozinke i korišćenjem trajne lozinke"),
        ("wayland_experiment_tip", "Wayland eksperiment savet"),
        ("Right click to select tabs", "Desni klik za izbor kartica"),
        ("Add to Address Book", "Dodaj u adresar"),
        ("Group", "Grupa"),
        ("Search", "Pretraga"),
        ].iter().cloned().collect();
}
