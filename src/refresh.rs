use std::sync::Arc;

use futures::Future;

use Error;

trait Refresh {
    type RefreshFuture: Future<Item=Arc<Self>, Error=Error>;

    fn refresh(&self) -> Self::RefreshFuture;
}
