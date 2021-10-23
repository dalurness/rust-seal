// #![allow(dead_code, unused_variables, unused_imports)]

extern crate seal;

use std::fmt::Debug;

use seal::pair::{
    Alignment, AlignmentSet, InMemoryAlignmentMatrix, NeedlemanWunsch, SmithWaterman, Step,
    Strategy,
};

fn trace(x_seq: &[char], y_seq: &[char], alignment: &Alignment) {
    let mut x_vec: Vec<char> = vec![];
    let mut y_vec: Vec<char> = vec![];
    for step in alignment.steps() {
        match step {
            Step::Align { x, y } => {
                print!("=");
                x_vec.push(x_seq[x]);
                y_vec.push(y_seq[y]);
            }
            Step::Delete { x } => {
                print!(">");
                x_vec.push(x_seq[x]);
                y_vec.push('-');
            }
            Step::Insert { y } => {
                print!("<");
                x_vec.push('-');
                y_vec.push(y_seq[y]);
            }
        }
    }

    println!();

    let x_str: String = x_vec.into_iter().collect();
    let y_str: String = y_vec.into_iter().collect();

    for (x, y) in x_str.chars().zip(y_str.chars()) {
        if x == y {
            print!("=");
        } else {
            print!("|");
        }
    }
    println!();

    println!("{}", x_str);
    println!("{}", y_str);
}

fn align<S: Strategy>(str_x: &str, str_y: &str, strategy: S)
where
    S: Debug,
{
    let sequence_x: Vec<char> = str_x.chars().collect();
    let sequence_y: Vec<char> = str_y.chars().collect();
    let alignment_set: Result<AlignmentSet<InMemoryAlignmentMatrix>, _> =
        AlignmentSet::new(sequence_x.len(), sequence_y.len(), strategy, |x, y| {
            sequence_x[x] == sequence_y[y]
        });

    match alignment_set {
        Ok(alignment_set) => {
            let local_alignment = alignment_set.local_alignment();
            println!("Local alignment:");
            trace(&sequence_x, &sequence_y, &local_alignment);
            println!();
            let global_alignment = alignment_set.global_alignment();
            println!("Global alignment:");
            trace(&sequence_x, &sequence_y, &global_alignment);
            println!("\n--------------------------\n");
        }
        Err(error) => {
            println!("Failed to generate alignment set due to error:");
            println!("{:?}", error);
        }
    }
}

fn main() {
    /*
    let human = "MDEEENHYVSQLREVYSSCDTTGTGFLDRQELTQLCLKLHLEQQLPVLLQTLLGNDHFARVNFEEFKEGF\
                 VAVLSSNAGVRPSDEDSSSLESAASSAIPPKYVNGSKWYGRRSRPELCDAATEARRVPEQQTQASLKSHL\
                 WRSASLESVESPKSDEEAEStKEAQNELFEAQGQLQTWDSEDFGSPQKSCSPSFDTPESQIRGVWEELGV\
                 GSSGHLSEQELAVVCQSVGLQGLEKEELEDLFNKLDQDGDGKVSLEEFQLGLFSHEPALLLESSTRVKPS\
                 KAWSHYQVPEESGCHTTTTSSLVSLCSSLRLFSSIDDGSGFAFPDQVLAMWTQEGIQNGREILQSLDFSV\
                 DEKVNLLELTWALDNELMTVDSAVQQAALACYHQELSYQQGQVEQLARERDKARQDLERAEKRNLEFVKE\
                 MDDCHSTLEQLTEKKIKHLEQGYRERLSLLRSEVEAERELFWEQAHRQRAALEWDVGRLQAEEAGLREKL\
                 TLALkENSRLQKEIVEVVEKLSDSERLALKLQKDLEFVLKDKLEPQSAELLAQEERFAAVLKEYELKCRD\
                 LQDRNDELQAELEGLWARLPKNRHSPSWSPDGRRRQLPGLGPAGISFLGNSAPVSIETELMMEQVKEHYQ\
                 DLRTQLETKVNYYEREIAALKRNFEKERKDMEQARRREVSVLEGQKADLEELHEKSQEVIWGLQEQLQDT\
                 ARGPEPEQMGLAPCCTQALCGLALRHHSHLQQIRREAEAELSGELSGLGALPARRDLtLELEEPPQGPLP\
                 RGSQRSEQLELERALKLQPCASEKRAQMCVSLALEEEELELARGKRVDGPSLEAEMQALPKDGLVAGSGQ\
                 EGTRGLLPLRPGCGERPLAWLAPGDGRESEEAAGAGPRRRQAQDTEATQSPAPAPAPASHGPSERWSRMQ\
                 PCGVDGDIVPKEPEPFGASAAGLEQPGARELPLLGTERDASQTQPRMWEPPLRPAASCRGQAERLQAIQE\
                 ERARSWSRGTQEQASEQQARAEGALEPGCHKHSVEVARRGSLPSHLQLADPQGSWQEQLAAPEEGETKIA\
                 LEREKDDMETKLLHLEDVVRALEKHVDLRENDRLEFHRLSEENTLLKNDLGRVRQELEAAESTHDAQRKE\
                 IEVLKKDKEKACSEMEVLNRQNQNYKDQLSQLNVRVLQLGQEASTHQAQNEEHRVTIQMLTQSLEEVVRS\
                 GQQQSDQIQKLRVELECLNQEHQSLQLPWSELTQTLEESQDQVQGAHLRLRQAQAQHLQEVRLVPQDRVA\
                 ELHRLLSLQGEQARRRLDAQREEHEKQLKATEERVEEAEMILKNMEMLLQEKVDKLKEQFEKNTKSDLLL\
                 KELYVENAHLVRALQATEEKQRGAEKQSRLLEEKVRALNKLVSRIAPAALSV";
    */
    let mouse = "MDNEEENHYVSRLRDVYSSCDTTGTGFLDQEELTQLCTKLGLEEQLPALLHILLGDDRLARVNFEEFKEG\
                 FVAVLSSGSGVEPSDEEGSSSESATSCAVPPKYMSGSKWYGRRSLPELGDSATATKYGSEQQAKGSVKPP\
                 LRRSAsLEsVEsLKSDEDAESAKEPQNELFEAQGQLRSWGCEVFGTLRKSCSPSFSTPENLVQGIWHELG\
                 IGSSGHLNEQELAVVCRSIGLHSLEKQELEELFSKLDQDGDGRVSLAEFQLGLFGHEPPSLPASSSLIKP\
                 NRLWSHYQEESGCHTTTTSSLVSVCSGLRLFSSVDDGSGFAFPEQVISAWAQEGIQNGREILQSLDFSVD\
                 EKVNLLELTWALDNELLTVDGVIQQAALACYRQELSYHQGQVDQLVQERDKARQDLEKAEKRNLDFVREM\
                 DDCHSALEQLTEKKIKHLEQEYRGRLSLLRSEVEMERELFWEQARRQRAVLEQDVGRLQAEETSLREKLT\
                 LALKENSRLQKEIIEVVEKLSDSEKLVLRLQSDLQFVLKDKLEPQSMELLAQEEQFTAILNDYELKCRDL\
                 QDRNDELQAELEGLRLRLPRSRQSPAGTPGTHRRRIPGRGPADNLFVGESTPVSLETEIMVEQMKEHYQE\
                 LRMQLETKVNYYEKEIEVMKRNFEKDKKEMEQAFQLEVSVLEGQKADLEALYAKSQEVILGLKEQLQDAA\
                 QSPEPAPAGLAHCCAQALCTLAQRLEVEMHLRHQDQLLQIRQEAEEELNQKLSWLEAQHAACCESLSLQH\
                 QCEKDQLLQTHLQRVKDLAAQLDLEKGRREEREQEVLAHCRRQQLKLQAVMSEEQARICRSFTLEKEKLE\
                 QTYREQVEGLVQEADVLRALLKNGTTVVSDQQERTPSSMSLGPDSRQQPTARQAVSPDGRTGAPAEWPGP\
                 EKAEGRDFPGQLCSIDAMPSPTPTLLSRRSSENLGVRDNHQRPLNAEEGAIPKEPEPSARTLTGQGQKLP\
                 LPVHPQMLEPSLGTTALDRKAASVGVQGQASEGPVGDGEGVQEAWLQFRGEATRMRPSLPCSELPNPQEA\
                 TVMPAMSESEMKDVKIKLLQLEDVVRALEKADSRESYRAELQRLSEENLVLKSDLGKIQLELETSESKNE\
                 VQRQEIEVLKRDKEQACCDLEELSTQTQKYKDEMSQLNCRVLQLEGEPSGLHTQKEENHGAIQVLMKKLE\
                 EAGCREEQQGDQIQNLKIELERVNEECQYLRLSQAELTESLEESRSQLYSVQLRLEAAQSQHGRIVQRLQ\
                 EQMSQLVPGARVAELQHLLNVKEEEARRLSAQQEEYRQQLKAREDQVEDAEARLRNVEWLLQEKVEELRK\
                 QFEKNTRSDLLLKELYVENAHLMKAVQLTEEKQRGAEKKNCVLEEKVRALNKLISKMAPASLSV";
    let rat = "MDNEEENHYVSRLRDVYSSCDTTGTGFLDQEELTQLCTKLGLEEQLPALLHILL\
               GDGRLARVNFEEFKEGFVAVLSSATGVEPSDEEGSSSESATSCAVPPKYMSGSK\
               WYGRRSLPELGDSATTTKCGSEQQAKGSVKPPLRRSASLESVESLKSDEDAESP\
               KEPQNELFEAQGQLRSWGCEVFGTPRKSCSPSFNTPENQVQGIWHELGVGSSGH\
               LNEQELAVVCRSIGLHGLEKQELEELFSKLDRDGDGRVSLAEFQLGLFGHEPPS\
               LPASSSLIKPNGPWSHYQEESGCHTTTTSSLVSVCSGLRLFSSVDDGSGFAFPE\
               QVISAWAQEGIQNGREILQSLDFNVDEKVNLLELTWALDNELLTVDGVIQQAAL\
               ACYRQELNFHQGQVEQLVQERDKARQDLEKAEKRNLDFVREMDDCHSALEQLTE\
               KKIKHLEQEYRGRLSLLRSEVEMERELFWEQARRQRAVLEQDVGRLQAEETSLR\
               EKLTLALKENSRLQKEIIEVVEKLSDSEKLVLRLQSDLQFVLKDKLEPQSMELL\
               AQEEQFTAILNDYELKCRDLQDRNDELQAELEGLRVRLPRSRQSPSGTPGTHRR\
               WTPGRGPADNLFVGESIPVSLETEIKMQQMKENYQELRMQLETKVNYyEKEIEV\
               MKRNFEKDKKEMEQAFQLEVSVLEGQKADLETLYAKSQEVILGLKEQLQDAARS\
               PEPAPAGLAPCCAQALCTLAQRLGVEMHLRHQDQLLQIRREAEEELNQKLSWLE\
               AQHAACCESLSLQHQCEKDQLLQTHLQRVKDLAAQLDLEKGWREEREQEVLAHC\
               RRQQLKLQADEEEQARICRSFTLEKEKLEQTYREQVEGLVQEADVLRALLKNGT\
               TVVSDQQERIPGSMYPGPDSRQQPPTWQTVSPDGRTGAPAEWPGPGRADGRDLP\
               GQLCSLDAVPSPTPTLLSRRSSESLDVRDNHQGPLSAEEGAVPKEPEPSARTLT\
               GQDQKLPLPIQPQMLEPWLGPAAVDRKPDSVRVQGQASEGPTGDDkGVQETPLQ\
               LRGETARMRPSLPYSELPNPQEAKVMSVMSESEMNDVKTKLLQLEDVVRALEKA\
               DSRESYRAELQRLSEENSVLKSDLGKIQLELGTSESRNEVQRQEIEVLKRDKEQ\
               ACFDLEELSTQTQKYKDEMSQLNCRILQLEGDSSGLHTQKEENHAAIQVLMKKL\
               EEAECREKQQGDQIKHLKIELERVNEECQRLRLSQAELTGSLEESQGQLHSVQL\
               RLEAAQSQHDRIVQGLQEQMSQLVPGARVAELQHLLSLREEEAERLNAQQEEYK\
               QQLKAREDQVEEAEARLHNVEWLLQEKVEELRKQFEKNTRSDLLLKELYVENAH\
               LMKAVQLTEEKQRGAEKKNCVLEEKVRALNKLISKMAPASLSV";

    let seq_a = mouse;
    let seq_b = rat;

    println!();

    let needleman_wunsch = NeedlemanWunsch::new(1, -1, -1, -1);

    align(seq_a, seq_b, needleman_wunsch.clone());
    println!();
    align(seq_b, seq_a, needleman_wunsch);

    let smith_waterman = SmithWaterman::new(2, -1, -1, -1);

    align(seq_a, seq_b, smith_waterman.clone());
    println!();
    align(seq_b, seq_a, smith_waterman);

    println!();
}
