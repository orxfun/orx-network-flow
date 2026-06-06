use crate::spaces::data::Geocode;

fn assert_distance_km(actual: f64, expected: f64, tolerance_km: f64) {
    let err = (actual - expected).abs();
    assert!(
        err <= tolerance_km,
        "distance mismatch: actual={actual:.3}km expected={expected:.3}km tol={tolerance_km:.3}km"
    );
}

#[test]
fn distance_km_bru_lej() {
    let bru = Geocode {
        lat: 50.901_389,
        lon: 4.484_444,
    };
    let lej = Geocode {
        lat: 51.423_889,
        lon: 12.236_389,
    };

    assert_distance_km(bru.distance_km(lej), 543.0, 10.0);
}

#[test]
fn distance_km_ams_mia() {
    let ams = Geocode {
        lat: 52.308_613,
        lon: 4.763_889,
    };
    let mia = Geocode {
        lat: 25.793_25,
        lon: -80.290_556,
    };

    assert_distance_km(ams.distance_km(mia), 7_440.0, 10.0);
}

#[test]
fn distance_km_jfk_hkg() {
    let jfk = Geocode {
        lat: 40.639_751,
        lon: -73.778_925,
    };
    let hkg = Geocode {
        lat: 22.308_889,
        lon: 113.914_444,
    };

    assert_distance_km(jfk.distance_km(hkg), 12_982.0, 20.0);
}

#[test]
fn distance_km_sin_syd() {
    let sin = Geocode {
        lat: 1.350_189,
        lon: 103.994_433,
    };
    let syd = Geocode {
        lat: -33.946_111,
        lon: 151.177_222,
    };

    assert_distance_km(sin.distance_km(syd), 6_301.0, 10.0);
}
