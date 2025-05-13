use std::io;

use std::io::BufWriter;
use std::io::Write;

use postgres::GenericClient;

pub fn tablename2pgcopy2writer<W, C>(
    trusted_table_name: &str,
    client: &mut C,
    writer: &mut W,
) -> Result<(), io::Error>
where
    C: GenericClient,
    W: Write,
{
    let query: String = format!("COPY {trusted_table_name} TO STDOUT WITH BINARY");
    let mut rdr = client.copy_out(&query).map_err(io::Error::other)?;
    io::copy(&mut rdr, writer)?;
    Ok(())
}

pub fn tablename2pgcopy2stdout<C>(trusted_table_name: &str, client: &mut C) -> Result<(), io::Error>
where
    C: GenericClient,
{
    let o = io::stdout();
    let mut l = o.lock();
    {
        let mut bw = BufWriter::new(&mut l);
        tablename2pgcopy2writer(trusted_table_name, client, &mut bw)?;
        bw.flush()?;
    }
    l.flush()
}
