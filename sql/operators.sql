CREATE OPERATOR ~~> (
    LEFTARG = text,
    RIGHTARG = text,
    FUNCTION = natord_gt,
    COMMUTATOR = ~~>
);

CREATE OPERATOR <~~ (
    LEFTARG = text,
    RIGHTARG = text,
    FUNCTION = natord_lt,
    COMMUTATOR = <~~
);

CREATE OPERATOR ~>= (
    LEFTARG = text,
    RIGHTARG = text,
    FUNCTION = natord_ge,
    COMMUTATOR = ~>=
);

CREATE OPERATOR <=~ (
    LEFTARG = text,
    RIGHTARG = text,
    FUNCTION = natord_le,
    COMMUTATOR = <=~
);

CREATE OPERATOR FAMILY natordfam
  USING btree;

CREATE OPERATOR CLASS natord_ops
  FOR TYPE text
  USING btree
  FAMILY natordfam AS
    OPERATOR 1 <~~        ,
    OPERATOR 2 <=~        ,
    OPERATOR 3 =          ,
    OPERATOR 4 ~>=        ,
    OPERATOR 5 ~~>        ,
    FUNCTION 1 natord_cmp ;
