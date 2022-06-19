#[derive(Debug,Default)]
struct Central {
       co2_by_kwh: f64,
       co2_at_construction: f64,
       kwc: f64, // kilowhatt pic
       taux_de_charge: f64,
       construction_year: u32,       
       lifetime: u32,
}

fn build_epr(construction_year: u32) -> Central {
   Central {
   	   co2_by_kwh: 6.0,
	   co2_at_construction: 0.0,
	   kwc: 1600000.0,
	   taux_de_charge: 80.0,	   
	   lifetime: 60,
	   construction_year: construction_year,
   }	   	     
}



