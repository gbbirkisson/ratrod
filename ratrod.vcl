vcl 4.1;

C{
#include "libratrod.h"
}C

backend localhost {
    .host = "127.0.0.1";
    .port = "8000";
}

sub something {
    C{
        ratrod();
        ratrod_fail(ctx);
        ratrod_ctx(ctx);
    }C
}

sub vcl_recv {
    call something;
}
