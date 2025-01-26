use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

use geo::{Point, Polygon, MultiPolygon, LineString};

#[derive(Serialize, Deserialize, Debug)]
pub struct baseCurve {
    #[serde(flatten)]
    pub base_curve: GmlCurvePropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cone {
    #[serde(flatten)]
    pub cone: GmlConeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct outDimension {
    #[serde(flatten)]
    pub out_dimension: positiveInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cylinder {
    #[serde(flatten)]
    pub cylinder: GmlCylinderType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeodesicString {
    #[serde(flatten)]
    pub geodesic_string: GmlGeodesicStringType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct multiplicity {
    #[serde(flatten)]
    pub multiplicity: nonNegativeInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PolyhedralSurface {
    #[serde(flatten)]
    pub polyhedral_surface: GmlPolyhedralSurfaceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct vectorAtStart {
    #[serde(flatten)]
    pub vector_at_start: GmlVectorType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct knot {
    #[serde(flatten)]
    pub knot: GmlKnotPropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bezier {
    #[serde(flatten)]
    pub bezier: GmlBezierType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct _Solid {
    #[serde(flatten)]
    pub __solid: GmlAbstractSolidType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct maxLength {
    #[serde(flatten)]
    pub max_length: GmlLengthType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct baseSurface {
    #[serde(flatten)]
    pub base_surface: GmlSurfacePropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct offsetBase {
    #[serde(flatten)]
    pub offset_base: GmlCurvePropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BSpline {
    #[serde(flatten)]
    pub b_spline: GmlBSplineType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct degree {
    #[serde(flatten)]
    pub degree: nonNegativeInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct rows {
    #[serde(flatten)]
    pub rows: integer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Triangle {
    #[serde(flatten)]
    pub triangle: GmlTriangleType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct startParameter {
    #[serde(flatten)]
    pub start_parameter: double,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct solidArrayProperty {
    #[serde(flatten)]
    pub solid_array_property: GmlSolidArrayPropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct endAngle {
    #[serde(flatten)]
    pub end_angle: GmlAngleType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct trianglePatches {
    #[serde(flatten)]
    pub triangle_patches: GmlTrianglePatchArrayPropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrientableSurface {
    #[serde(flatten)]
    pub orientable_surface: GmlOrientableSurfaceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct inDimension {
    #[serde(flatten)]
    pub in_dimension: positiveInteger,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sphere {
    #[serde(flatten)]
    pub sphere: GmlSphereType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OffsetCurve {
    #[serde(flatten)]
    pub offset_curve: GmlOffsetCurveType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TriangulatedSurface {
    #[serde(flatten)]
    pub triangulated_surface: GmlTriangulatedSurfaceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Curve {
    #[serde(flatten)]
    pub curve: GmlCurveType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Clothoid {
    #[serde(flatten)]
    pub clothoid: GmlClothoidType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Knot {
    #[serde(flatten)]
    pub knot: GmlKnotType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct controlPoint {
    #[serde(flatten)]
    pub control_point: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct bulge {
    #[serde(flatten)]
    pub bulge: double,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct stopLines {
    #[serde(flatten)]
    pub stop_lines: GmlLineStringSegmentArrayPropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct breakLines {
    #[serde(flatten)]
    pub break_lines: GmlLineStringSegmentArrayPropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LineStringSegment {
    #[serde(flatten)]
    pub line_string_segment: GmlLineStringSegmentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct endParameter {
    #[serde(flatten)]
    pub end_parameter: double,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Arc {
    #[serde(flatten)]
    pub arc: GmlArcType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct normal {
    #[serde(flatten)]
    pub normal: GmlVectorType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct startAngle {
    #[serde(flatten)]
    pub start_angle: GmlAngleType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct location {
    #[serde(flatten)]
    pub location: GmlDirectPositionType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct vectorAtEnd {
    #[serde(flatten)]
    pub vector_at_end: GmlVectorType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct patches {
    #[serde(flatten)]
    pub patches: GmlSurfacePatchArrayPropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PolygonPatch {
    #[serde(flatten)]
    pub polygon_patch: GmlPolygonPatchType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct _CurveSegment {
    #[serde(flatten)]
    pub __curve_segment: GmlAbstractCurveSegmentType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct polygonPatches {
    #[serde(flatten)]
    pub polygon_patches: GmlPolygonPatchArrayPropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct columns {
    #[serde(flatten)]
    pub columns: integer,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct refLocation {
    #[serde(flatten)]
    pub ref_location: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArcStringByBulge {
    #[serde(flatten)]
    pub arc_string_by_bulge: GmlArcStringByBulgeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct row {
    #[serde(flatten)]
    pub row: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Rectangle {
    #[serde(flatten)]
    pub rectangle: GmlRectangleType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Surface {
    #[serde(flatten)]
    pub surface: GmlSurfaceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Circle {
    #[serde(flatten)]
    pub circle: GmlCircleType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CircleByCenterPoint {
    #[serde(flatten)]
    pub circle_by_center_point: GmlCircleByCenterPointType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct _SurfacePatch {
    #[serde(flatten)]
    pub __surface_patch: GmlAbstractSurfacePatchType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct curveMember {
    #[serde(flatten)]
    pub curve_member: GmlCurvePropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AffinePlacement {
    #[serde(flatten)]
    pub affine_placement: GmlAffinePlacementType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct solidProperty {
    #[serde(flatten)]
    pub solid_property: GmlSolidPropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CubicSpline {
    #[serde(flatten)]
    pub cubic_spline: GmlCubicSplineType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct scaleFactor {
    #[serde(flatten)]
    pub scale_factor: decimal,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct refDirection {
    #[serde(flatten)]
    pub ref_direction: GmlVectorType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArcString {
    #[serde(flatten)]
    pub arc_string: GmlArcStringType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct value {
    #[serde(flatten)]
    pub value: double,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct weight {
    #[serde(flatten)]
    pub weight: double,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct radius {
    #[serde(flatten)]
    pub radius: GmlLengthType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArcByBulge {
    #[serde(flatten)]
    pub arc_by_bulge: GmlArcByBulgeType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrientableCurve {
    #[serde(flatten)]
    pub orientable_curve: GmlOrientableCurveType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tin {
    #[serde(flatten)]
    pub tin: GmlTinType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct _ParametricCurveSurface {
    #[serde(flatten)]
    pub __parametric_curve_surface: GmlAbstractParametricCurveSurfaceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct segments {
    #[serde(flatten)]
    pub segments: GmlCurveSegmentArrayPropertyType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ring {
    #[serde(flatten)]
    pub ring: GmlRingType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ArcByCenterPoint {
    #[serde(flatten)]
    pub arc_by_center_point: GmlArcByCenterPointType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Geodesic {
    #[serde(flatten)]
    pub geodesic: GmlGeodesicType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct _GriddedSurface {
    #[serde(flatten)]
    pub __gridded_surface: GmlAbstractGriddedSurfaceType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Solid {
    #[serde(flatten)]
    pub solid: GmlSolidType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolidType {
    #[serde(flatten)]
    pub base: GmlAbstractSolidType,
    #[serde(rename = "exterior", skip_serializing_if = "Option::is_none")]
    pub exterior: Option<SurfacePropertyType>,
    #[serde(rename = "interior", skip_serializing_if = "Option::is_none")]
    pub interior: Option<Vec<SurfacePropertyType>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurveInterpolationType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RingPropertyType {
    #[serde(rename = "Ring")]
    pub gml_ring: GmlRing,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KnotTypesType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbstractCurveSegmentType {
    #[serde(rename = "numDerivativesAtStart")]
    pub num_derivatives_at_start: i32,
    #[serde(rename = "numDerivativesAtEnd")]
    pub num_derivatives_at_end: i32,
    #[serde(rename = "numDerivativeInterior")]
    pub num_derivative_interior: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AffinePlacementType {
    #[serde(rename = "location")]
    pub location: DirectPositionType,
    #[serde(rename = "refDirection")]
    pub ref_direction: Vec<VectorType>,
    #[serde(rename = "inDimension")]
    pub in_dimension: u32,
    #[serde(rename = "outDimension")]
    pub out_dimension: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolyhedralSurfaceType {
    #[serde(flatten)]
    pub base: GmlSurfaceType,
    #[serde(rename = "StandardObjectProperties")]
    pub gml_standard_object_properties: GmlStandardObjectProperties,
    #[serde(rename = "polygonPatches")]
    pub gmlpolygon_patches: GmlpolygonPatches,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonPatchArrayPropertyType {
    #[serde(flatten)]
    pub base: GmlSurfacePatchArrayPropertyType,
    #[serde(rename = "PolygonPatch")]
    pub gml_polygon_patch: GmlPolygonPatch,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArcByBulgeType {
    #[serde(flatten)]
    pub base: GmlArcStringByBulgeType,
    #[serde(rename = "pos")]
    pub gmlpos: Gmlpos,
    #[serde(rename = "pointProperty")]
    pub gmlpoint_property: GmlpointProperty,
    #[serde(rename = "pointRep")]
    pub gmlpoint_rep: GmlpointRep,
    #[serde(rename = "posList")]
    pub gmlpos_list: GmlposList,
    #[serde(rename = "coordinates")]
    pub gmlcoordinates: Gmlcoordinates,
    #[serde(rename = "bulge")]
    pub bulge: f64,
    #[serde(rename = "normal")]
    pub normal: VectorType,
    #[serde(rename = "numArc")]
    pub num_arc: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BSplineType {
    #[serde(flatten)]
    pub base: GmlAbstractCurveSegmentType,
    #[serde(rename = "pos")]
    pub gmlpos: Gmlpos,
    #[serde(rename = "pointProperty")]
    pub gmlpoint_property: GmlpointProperty,
    #[serde(rename = "pointRep")]
    pub gmlpoint_rep: GmlpointRep,
    #[serde(rename = "posList")]
    pub gmlpos_list: GmlposList,
    #[serde(rename = "coordinates")]
    pub gmlcoordinates: Gmlcoordinates,
    #[serde(rename = "degree")]
    pub degree: u32,
    #[serde(rename = "knot")]
    pub knot: Vec<KnotPropertyType>,
    #[serde(rename = "interpolation")]
    pub interpolation: CurveInterpolationType,
    #[serde(rename = "isPolynomial")]
    pub is_polynomial: bool,
    #[serde(rename = "knotType")]
    pub knot_type: KnotTypesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrianglePatchArrayPropertyType {
    #[serde(flatten)]
    pub base: GmlSurfacePatchArrayPropertyType,
    #[serde(rename = "Triangle")]
    pub gml_triangle: GmlTriangle,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TriangulatedSurfaceType {
    #[serde(flatten)]
    pub base: GmlSurfaceType,
    #[serde(rename = "StandardObjectProperties")]
    pub gml_standard_object_properties: GmlStandardObjectProperties,
    #[serde(rename = "trianglePatches")]
    pub gmltriangle_patches: GmltrianglePatches,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbstractSolidType {
    #[serde(flatten)]
    pub base: GmlAbstractGeometricPrimitiveType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolidArrayPropertyType {
    #[serde(rename = "_Solid")]
    pub gml__solid: Gml_Solid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircleByCenterPointType {
    #[serde(flatten)]
    pub base: GmlArcByCenterPointType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbstractGriddedSurfaceType {
    #[serde(flatten)]
    pub base: GmlAbstractParametricCurveSurfaceType,
    #[serde(rename = "PointGrid")]
    pub gml_point_grid: GmlPointGrid,
    #[serde(rename = "rows", skip_serializing_if = "Option::is_none")]
    pub rows: Option<i32>,
    #[serde(rename = "columns", skip_serializing_if = "Option::is_none")]
    pub columns: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurveSegmentArrayPropertyType {
    #[serde(rename = "_CurveSegment", skip_serializing_if = "Option::is_none")]
    pub gml__curve_segment: Option<Vec<Gml_CurveSegment>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KnotPropertyType {
    #[serde(rename = "Knot")]
    pub knot: KnotType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SurfacePatchArrayPropertyType {
    #[serde(rename = "_SurfacePatch")]
    pub gml__surface_patch: Gml_SurfacePatch,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CylinderType {
    #[serde(flatten)]
    pub base: GmlAbstractGriddedSurfaceType,
    #[serde(rename = "horizontalCurveType")]
    pub horizontal_curve_type: CurveInterpolationType,
    #[serde(rename = "verticalCurveType")]
    pub vertical_curve_type: CurveInterpolationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SurfaceInterpolationType {
    pub base: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CircleType {
    #[serde(flatten)]
    pub base: GmlArcType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SurfaceType {
    #[serde(flatten)]
    pub base: GmlAbstractSurfaceType,
    #[serde(rename = "patches")]
    pub gmlpatches: Gmlpatches,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RectangleType {
    #[serde(flatten)]
    pub base: GmlAbstractSurfacePatchType,
    #[serde(rename = "exterior")]
    pub gmlexterior: Gmlexterior,
    #[serde(rename = "interpolation")]
    pub interpolation: SurfaceInterpolationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CubicSplineType {
    #[serde(flatten)]
    pub base: GmlAbstractCurveSegmentType,
    #[serde(rename = "pos")]
    pub gmlpos: Gmlpos,
    #[serde(rename = "pointProperty")]
    pub gmlpoint_property: GmlpointProperty,
    #[serde(rename = "pointRep")]
    pub gmlpoint_rep: GmlpointRep,
    #[serde(rename = "posList")]
    pub gmlpos_list: GmlposList,
    #[serde(rename = "coordinates")]
    pub gmlcoordinates: Gmlcoordinates,
    #[serde(rename = "vectorAtStart")]
    pub vector_at_start: VectorType,
    #[serde(rename = "vectorAtEnd")]
    pub vector_at_end: VectorType,
    #[serde(rename = "interpolation")]
    pub interpolation: CurveInterpolationType,
    #[serde(rename = "degree")]
    pub degree: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SphereType {
    #[serde(flatten)]
    pub base: GmlAbstractGriddedSurfaceType,
    #[serde(rename = "horizontalCurveType")]
    pub horizontal_curve_type: CurveInterpolationType,
    #[serde(rename = "verticalCurveType")]
    pub vertical_curve_type: CurveInterpolationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LineStringSegmentArrayPropertyType {
    #[serde(rename = "LineStringSegment", skip_serializing_if = "Option::is_none")]
    pub gml_line_string_segment: Option<Vec<GmlLineStringSegment>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClothoidType {
    #[serde(flatten)]
    pub base: GmlAbstractCurveSegmentType,
    #[serde(rename = "refLocation")]
    pub ref_location: String,
    #[serde(rename = "AffinePlacement")]
    pub gml_affine_placement: GmlAffinePlacement,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrientableSurfaceType {
    #[serde(flatten)]
    pub base: GmlAbstractSurfaceType,
    #[serde(rename = "baseSurface")]
    pub gmlbase_surface: GmlbaseSurface,
    #[serde(rename = "orientation")]
    pub orientation: SignType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArcByCenterPointType {
    #[serde(flatten)]
    pub base: GmlAbstractCurveSegmentType,
    #[serde(rename = "pos")]
    pub gmlpos: Gmlpos,
    #[serde(rename = "pointProperty")]
    pub gmlpoint_property: GmlpointProperty,
    #[serde(rename = "pointRep")]
    pub gmlpoint_rep: GmlpointRep,
    #[serde(rename = "posList")]
    pub gmlpos_list: GmlposList,
    #[serde(rename = "coordinates")]
    pub gmlcoordinates: Gmlcoordinates,
    #[serde(rename = "radius")]
    pub radius: LengthType,
    #[serde(rename = "startAngle", skip_serializing_if = "Option::is_none")]
    pub start_angle: Option<AngleType>,
    #[serde(rename = "endAngle", skip_serializing_if = "Option::is_none")]
    pub end_angle: Option<AngleType>,
    #[serde(rename = "interpolation")]
    pub interpolation: CurveInterpolationType,
    #[serde(rename = "numArc")]
    pub num_arc: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TriangleType {
    #[serde(flatten)]
    pub base: GmlAbstractSurfacePatchType,
    #[serde(rename = "exterior")]
    pub gmlexterior: Gmlexterior,
    #[serde(rename = "interpolation")]
    pub interpolation: SurfaceInterpolationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OffsetCurveType {
    #[serde(flatten)]
    pub base: GmlAbstractCurveSegmentType,
    #[serde(rename = "offsetBase")]
    pub offset_base: CurvePropertyType,
    #[serde(rename = "distance")]
    pub distance: LengthType,
    #[serde(rename = "refDirection", skip_serializing_if = "Option::is_none")]
    pub ref_direction: Option<VectorType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KnotType {
    #[serde(rename = "value")]
    pub value: f64,
    #[serde(rename = "multiplicity")]
    pub multiplicity: u32,
    #[serde(rename = "weight")]
    pub weight: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BezierType {
    #[serde(flatten)]
    pub base: GmlBSplineType,
    #[serde(rename = "pos")]
    pub gmlpos: Gmlpos,
    #[serde(rename = "pointProperty")]
    pub gmlpoint_property: GmlpointProperty,
    #[serde(rename = "pointRep")]
    pub gmlpoint_rep: GmlpointRep,
    #[serde(rename = "posList")]
    pub gmlpos_list: GmlposList,
    #[serde(rename = "coordinates")]
    pub gmlcoordinates: Gmlcoordinates,
    #[serde(rename = "degree")]
    pub degree: u32,
    #[serde(rename = "knot")]
    pub knot: Vec<KnotPropertyType>,
    #[serde(rename = "interpolation")]
    pub interpolation: CurveInterpolationType,
    #[serde(rename = "isPolynomial")]
    pub is_polynomial: bool,
    #[serde(rename = "knotType")]
    pub knot_type: KnotTypesType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TinType {
    #[serde(flatten)]
    pub base: GmlTriangulatedSurfaceType,
    #[serde(rename = "stopLines", skip_serializing_if = "Option::is_none")]
    pub stop_lines: Option<Vec<LineStringSegmentArrayPropertyType>>,
    #[serde(rename = "breakLines", skip_serializing_if = "Option::is_none")]
    pub break_lines: Option<Vec<LineStringSegmentArrayPropertyType>>,
    #[serde(rename = "maxLength")]
    pub max_length: LengthType,
    #[serde(rename = "controlPoint")]
    pub control_point: String,
    #[serde(rename = "posList")]
    pub gmlpos_list: GmlposList,
    #[serde(rename = "geometricPositionGroup")]
    pub gmlgeometric_position_group: Vec<GmlgeometricPositionGroup>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LineStringSegmentType {
    #[serde(flatten)]
    pub base: GmlAbstractCurveSegmentType,
    #[serde(rename = "pos")]
    pub gmlpos: Gmlpos,
    #[serde(rename = "pointProperty")]
    pub gmlpoint_property: GmlpointProperty,
    #[serde(rename = "pointRep")]
    pub gmlpoint_rep: GmlpointRep,
    #[serde(rename = "posList")]
    pub gmlpos_list: GmlposList,
    #[serde(rename = "coordinates")]
    pub gmlcoordinates: Gmlcoordinates,
    #[serde(rename = "interpolation")]
    pub interpolation: CurveInterpolationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RingType {
    #[serde(flatten)]
    pub base: GmlAbstractRingType,
    #[serde(rename = "curveMember")]
    pub gmlcurve_member: Vec<GmlcurveMember>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrientableCurveType {
    #[serde(flatten)]
    pub base: GmlAbstractCurveType,
    #[serde(rename = "baseCurve")]
    pub gmlbase_curve: GmlbaseCurve,
    #[serde(rename = "orientation")]
    pub orientation: SignType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeodesicType {
    #[serde(flatten)]
    pub base: GmlGeodesicStringType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AbstractParametricCurveSurfaceType {
    #[serde(flatten)]
    pub base: GmlAbstractSurfacePatchType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConeType {
    #[serde(flatten)]
    pub base: GmlAbstractGriddedSurfaceType,
    #[serde(rename = "horizontalCurveType")]
    pub horizontal_curve_type: CurveInterpolationType,
    #[serde(rename = "verticalCurveType")]
    pub vertical_curve_type: CurveInterpolationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArcStringType {
    #[serde(flatten)]
    pub base: GmlAbstractCurveSegmentType,
    #[serde(rename = "pos")]
    pub gmlpos: Gmlpos,
    #[serde(rename = "pointProperty")]
    pub gmlpoint_property: GmlpointProperty,
    #[serde(rename = "pointRep")]
    pub gmlpoint_rep: GmlpointRep,
    #[serde(rename = "posList")]
    pub gmlpos_list: GmlposList,
    #[serde(rename = "coordinates")]
    pub gmlcoordinates: Gmlcoordinates,
    #[serde(rename = "interpolation")]
    pub interpolation: CurveInterpolationType,
    #[serde(rename = "numArc")]
    pub num_arc: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolygonPatchType {
    #[serde(flatten)]
    pub base: GmlAbstractSurfacePatchType,
    #[serde(rename = "exterior", skip_serializing_if = "Option::is_none")]
    pub gmlexterior: Option<Gmlexterior>,
    #[serde(rename = "interior", skip_serializing_if = "Option::is_none")]
    pub gmlinterior: Option<Vec<Gmlinterior>>,
    #[serde(rename = "interpolation")]
    pub interpolation: SurfaceInterpolationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurveType {
    #[serde(flatten)]
    pub base: GmlAbstractCurveType,
    #[serde(rename = "segments")]
    pub gmlsegments: Gmlsegments,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArcType {
    #[serde(flatten)]
    pub base: GmlArcStringType,
    #[serde(rename = "pos")]
    pub gmlpos: Gmlpos,
    #[serde(rename = "pointProperty")]
    pub gmlpoint_property: GmlpointProperty,
    #[serde(rename = "pointRep")]
    pub gmlpoint_rep: GmlpointRep,
    #[serde(rename = "posList")]
    pub gmlpos_list: GmlposList,
    #[serde(rename = "coordinates")]
    pub gmlcoordinates: Gmlcoordinates,
    #[serde(rename = "numArc")]
    pub num_arc: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArcStringByBulgeType {
    #[serde(flatten)]
    pub base: GmlAbstractCurveSegmentType,
    #[serde(rename = "pos")]
    pub gmlpos: Gmlpos,
    #[serde(rename = "pointProperty")]
    pub gmlpoint_property: GmlpointProperty,
    #[serde(rename = "pointRep")]
    pub gmlpoint_rep: GmlpointRep,
    #[serde(rename = "posList")]
    pub gmlpos_list: GmlposList,
    #[serde(rename = "coordinates")]
    pub gmlcoordinates: Gmlcoordinates,
    #[serde(rename = "bulge")]
    pub bulge: Vec<f64>,
    #[serde(rename = "normal")]
    pub normal: Vec<VectorType>,
    #[serde(rename = "interpolation")]
    pub interpolation: CurveInterpolationType,
    #[serde(rename = "numArc")]
    pub num_arc: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeodesicStringType {
    #[serde(flatten)]
    pub base: GmlAbstractCurveSegmentType,
    #[serde(rename = "posList")]
    pub gmlpos_list: GmlposList,
    #[serde(rename = "geometricPositionGroup")]
    pub gmlgeometric_position_group: Vec<GmlgeometricPositionGroup>,
    #[serde(rename = "interpolation")]
    pub interpolation: CurveInterpolationType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolidPropertyType {
    #[serde(rename = "_Solid")]
    pub gml__solid: Gml_Solid,
}

